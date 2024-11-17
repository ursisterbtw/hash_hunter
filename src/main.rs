mod config;
use crate::config::Config;
use bytes::BytesMut;
use chrono::Utc;
use clap::Parser;
use colored::*;
use dashmap::DashMap;
use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::OsRng;
use regex::Regex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// eth addy gen in rust, zooms
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // prefix of the eth address
    #[arg(short = 'p', long, default_value = "69")]
    start_pattern: String,

    // suffix of the eth address
    #[arg(short = 'e', long, default_value = "69696969")]
    end_pattern: String,

    // enable EIP-55 checksum
    #[arg(short = 'c', long, default_value_t = true)]
    checksum: bool,

    // # of attempts between progress logs
    #[arg(short = 's', long, default_value_t = 50_000)]
    step: u64,

    // max # of attempts
    #[arg(short = 'm', long, default_value_t = 10_000_000_000)]
    max_tries: u64,

    // logging interval in ms
    #[arg(short = 'i', long, default_value_t = 15_000)]
    log_interval: u64,

    // minimum number of zeros in the address
    #[arg(short = 'z', long, default_value_t = 0)]
    min_zeros: usize,

    // regex pattern to match in the address
    #[arg(short = 'r', long, default_value = "")]
    regex_pattern: String,

    // skip confirmation prompt for docker
    #[arg(short = 'y', long, default_value_t = false)]
    skip_confirmation: bool,
}

struct VanityResult {
    address: String,
    priv_key: String,
    attempts: u64,
}

fn main() {
    print_startup_screen();

    // Load config first - rename the variable to cfg to avoid confusion with the module
    let cfg = Config::load();
    println!("Running {} v{}", cfg.app.name, cfg.app.version);
    println!("{}", cfg.app.description);
    println!("Warning: {}", cfg.app.warning);

    if cfg.search.validation.verify_addresses {
        println!("Address verification enabled");
    }

    let thread_count = match cfg.performance.threads.as_str() {
        "auto" => num_cpus::get(),
        count => count.parse().unwrap_or_else(|_| num_cpus::get()),
    };
    println!("Using {} threads", thread_count);

    // Use config values as defaults for Args
    let args = Args::parse();

    // Override args with config if provided
    let start_pattern = if args.start_pattern == "69" {
        cfg.search.patterns.start.clone()
    } else {
        args.start_pattern.clone()
    };
    let end_pattern = if args.end_pattern == "69696969" {
        cfg.search.patterns.end.clone()
    } else {
        args.end_pattern.clone()
    };
    let use_checksum = args.checksum || cfg.search.validation.use_checksum;
    let step = args.step.max(cfg.performance.step_size);
    let max_tries = args.max_tries.max(cfg.performance.max_tries);
    let log_interval = args.log_interval.max(cfg.performance.log_interval_ms);
    let min_zeros = args.min_zeros.max(cfg.search.validation.min_zeros);
    let regex_pattern = if !args.regex_pattern.is_empty() {
        Some(Arc::new(
            Regex::new(&args.regex_pattern).expect("Invalid regex pattern"),
        ))
    } else if !cfg.search.patterns.regex.is_empty() {
        Some(Arc::new(
            Regex::new(&cfg.search.patterns.regex).expect("Invalid regex pattern in config"),
        ))
    } else {
        None
    };
    let skip_confirmation = args.skip_confirmation || cfg.security.skip_confirmation;

    // add a confirmation prompt
    if !confirm_start(&Args {
        start_pattern: start_pattern.clone(),
        end_pattern: end_pattern.clone(),
        checksum: use_checksum,
        step,
        max_tries,
        log_interval,
        min_zeros,
        regex_pattern: args.regex_pattern.clone(), // or use config pattern
        skip_confirmation,
    }) {
        println!("Operation cancelled by user.");
        return;
    }

    println!("Starting Vanity Address Generator 🧪");
    println!("Prefix: {}", start_pattern.bright_green());
    println!("Suffix: {}", end_pattern.bright_green());
    println!(
        "Checksum: {}",
        if use_checksum {
            "✅".green()
        } else {
            "❌".red()
        }
    );
    println!("Minimum Zeros: {}", min_zeros.to_string().yellow());
    println!("Step: {}", step.to_string().yellow());
    println!("Max Tries: {}", max_tries.to_string().yellow());
    println!("Log Interval (ms): {}", log_interval.to_string().yellow());
    println!("Regex Pattern: {}", args.regex_pattern.yellow());

    let found = Arc::new(AtomicBool::new(false));
    let result_map = Arc::new(DashMap::new());
    let total_attempts = Arc::new(DashMap::new());
    total_attempts.insert("attempts", 0u64);

    let start_time = Instant::now();

    let progress_bar = Arc::new(setup_progress_bar(max_tries));

    // start logs
    {
        let total_attempts = Arc::clone(&total_attempts);
        let found = Arc::clone(&found);
        let progress_bar = Arc::clone(&progress_bar);
        std::thread::spawn(move || {
            let mut last_attempts = 0u64;
            while !found.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(log_interval));
                let attempts = total_attempts.get("attempts").map(|a| *a).unwrap_or(0);
                progress_bar.set_position(attempts);

                // Add rate calculation
                let rate = (attempts - last_attempts) as f64 / (log_interval as f64 / 1000.0);
                println!("Rate: {:.2} attempts/sec, Total: {}", rate, attempts);
                last_attempts = attempts;
            }
        });
    }

    // check cores to determine cpu count, then create threads
    let num_threads = num_cpus::get();

    // create scope for threads
    rayon::scope(|s| {
        for _ in 0..num_threads {
            let start_pattern = start_pattern.clone();
            let end_pattern = end_pattern.clone();
            let found = Arc::clone(&found);
            let result_map = Arc::clone(&result_map);
            let total_attempts = Arc::clone(&total_attempts);
            let regex_pattern = regex_pattern.clone(); // Clone the regex pattern for each thread

            s.spawn(move |_| {
                let secp = Secp256k1::new();
                let mut rng = OsRng;
                let mut local_attempts = 0u64;

                // Pre-allocate strings
                let mut address = String::with_capacity(40);
                let mut final_address = String::with_capacity(40);
                let mut priv_key_hex = String::with_capacity(64);

                while !found.load(Ordering::Relaxed)
                    && total_attempts.get("attempts").map_or(0, |a| *a) < max_tries
                {
                    // key gen
                    let secret_key = SecretKey::new(&mut rng);

                    // compute pubkey and get last 20 bytes directly
                    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
                    let serialized_pub = public_key.serialize_uncompressed();
                    let hash = Keccak256::digest(&serialized_pub[1..]);

                    // Reuse pre-allocated strings
                    address.clear();
                    address.push_str(&hex::encode(&hash[12..]));

                    final_address.clear();
                    if use_checksum {
                        to_checksum_address_into(&address, &mut final_address);
                    } else {
                        final_address.push_str(&address);
                    }

                    // Check conditions
                    if final_address.starts_with(&start_pattern)
                        && final_address.ends_with(&end_pattern)
                        && final_address.matches('0').count() >= min_zeros
                        && regex_pattern
                            .as_ref()
                            .map_or(true, |re| re.is_match(&final_address))
                    {
                        // Found match - reuse priv_key string
                        priv_key_hex.clear();
                        let _ = hex::encode_to_slice(
                            secret_key.as_ref(),
                            &mut BytesMut::from(priv_key_hex.as_bytes()),
                        );

                        result_map.insert(
                            "result",
                            VanityResult {
                                address: format!("0x{}", final_address),
                                priv_key: priv_key_hex.clone(),
                                attempts: total_attempts.get("attempts").map_or(0, |a| *a),
                            },
                        );
                        found.store(true, Ordering::Relaxed);
                        break;
                    }

                    local_attempts += 1;
                    if local_attempts >= step {
                        total_attempts
                            .entry("attempts")
                            .and_modify(|a| *a += local_attempts);
                        local_attempts = 0;
                    }
                }

                if local_attempts > 0 {
                    total_attempts
                        .entry("attempts")
                        .and_modify(|a| *a += local_attempts);
                }
            });
        }
    });

    // create 'gen' directory if it doesn't exist
    std::fs::create_dir_all("gen").expect("Failed to create 'gen' directory");

    // update progress bar one last time
    let final_attempts = total_attempts.get("attempts").map(|a| *a).unwrap_or(0);
    progress_bar.set_position(final_attempts);
    progress_bar.finish_with_message("Search completed");

    // check if a result was found
    if let Some(result) = result_map.get("result") {
        println!("\n{}", "🌀 Address found! 🌀".bright_green().bold());
        println!("Address: {}", result.address.bright_green());
        println!("Private Key: {}", result.priv_key.yellow());
        println!("Total attempts: {}", result.attempts.to_string().cyan());

        // verify the generated address
        if verify_address(&result.address, &result.priv_key) {
            println!("{}", "Address verification: PASSED ✅".green());
        } else {
            println!("{}", "Address verification: FAILED ❌".red());
            println!(
                "{}",
                "Warning: The generated address does not match the private key!"
                    .red()
                    .bold()
            );
        }

        // create a filename based on the public key
        let filename = format!("gen/{}.json", result.address);

        // create a JSON object
        let json_output = serde_json::json!({
            "address": result.address,
            "privateKey": result.priv_key,
            "totalAttempts": result.attempts
        });

        // write to file with restricted permissions
        std::fs::write(
            &filename,
            serde_json::to_string_pretty(&json_output).unwrap(),
        )
        .expect("Unable to write to file");

        // Set file permissions to read/write for the owner only
        //#[cfg(unix)]
        //{
        //    use std::fs::Permissions;
        //    use std::os::unix::fs::PermissionsExt;
        //
        //    fs::set_permissions(&filename, Permissions::from_mode(0o600))
        //        .expect("Failed to set file permissions");
        //}

        println!(
            "{}",
            format!(
                "Address, private key, and attempt count saved to {} 💾",
                filename
            )
            .bright_blue()
        );

        // add entropy estimation
        print_entropy_estimation(&result.address);

        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("gen/hunter.log")
            .expect("Failed to open log file");

        let mut log_writer = std::io::BufWriter::new(log_file);

        // Log startup
        writeln!(
            log_writer,
            "[{}] Starting hash_hunter with prefix: {}, suffix: {}",
            Utc::now(),
            start_pattern,
            end_pattern
        )
        .expect("Failed to write to log");
        log_writer.flush().expect("Failed to flush log");

        // When a match is found, log it
        writeln!(
            log_writer,
            "[{}] Found match! Address: {}, Attempts: {}",
            Utc::now(),
            result.address,
            result.attempts
        )
        .expect("Failed to write to log");
        log_writer.flush().expect("Failed to flush log");

        // Create a success marker file
        std::fs::write(
            "gen/SUCCESS",
            format!("Found address: {}\n", result.address),
        )
        .expect("Failed to write success marker");
    } else {
        println!(
            "{}",
            "Maximum attempts reached without finding a matching address. 😭".red()
        );
    }

    let elapsed = start_time.elapsed();
    println!(
        "Total time elapsed: {:.2?} for {} attempts ⏱️",
        elapsed,
        total_attempts
            .get("attempts")
            .map_or(0, |a| *a)
            .to_string()
            .cyan()
    );
}

fn print_startup_screen() {
    println!("\n\n");
    println!("\n");
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║           hash_hunter addy generator from ursister            ║".bright_cyan()
    );
    println!(
        "{}",
        "║                     🦖 kingz on top 🦖                        ║".bright_cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!("\n");
}

fn verify_address(address: &str, private_key: &str) -> bool {
    let secp = Secp256k1::new();
    let secret_key = match hex::decode(private_key) {
        Ok(bytes) => match SecretKey::from_slice(&bytes) {
            Ok(key) => key,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let serialized_pub = public_key.serialize_uncompressed();
    let hash = Keccak256::digest(&serialized_pub[1..]);
    let generated_address = format!("0x{}", hex::encode(&hash[12..]));

    address.to_lowercase() == generated_address.to_lowercase()
}

fn _verify_checksum(checksum_address: &str) -> bool {
    // First verify the checksum format if address has mixed case
    if checksum_address.chars().any(|c| c.is_ascii_uppercase()) {
        let address_without_prefix = &checksum_address[2..];
        let mut message = [0u8; 32];
        let mut keccak = Keccak256::new();
        keccak.update(address_without_prefix.to_lowercase().as_bytes());
        keccak.finalize_into((&mut message).into());

        let mut checksummed = String::with_capacity(42);
        checksummed.push_str("0x");

        for (i, ch) in address_without_prefix.chars().enumerate() {
            let nibble = u8::from_str_radix(&message[i / 2].to_string(), 16).unwrap();
            let should_be_uppercase = nibble & 0x8 == 0x8;

            if ch.is_ascii_uppercase() != should_be_uppercase {
                return false;
            }
            checksummed.push(ch);
        }

        if checksummed != checksum_address {
            return false;
        }
    }

    true
}

fn confirm_start(args: &Args) -> bool {
    if args.skip_confirmation {
        return true;
    }

    println!("\nAre you sure you want to start with these parameters? (y/n)");
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() == "y"
}

fn setup_progress_bar(max_tries: u64) -> ProgressBar {
    let pb = ProgressBar::new(max_tries);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}

fn print_entropy_estimation(address: &str) {
    let address_without_prefix = address.trim_start_matches("0x");
    let entropy_bits = address_without_prefix.len() * 4; // each hex character represents 4 bits
    println!("Estimated entropy: {} bits", entropy_bits);

    let years_to_crack = calculate_years_to_crack(entropy_bits);
    println!("Estimated time to crack: {:.2e} years", years_to_crack);
}

fn calculate_years_to_crack(entropy_bits: usize) -> f64 {
    let guesses_per_second = 1e12; // assume 1 trillion guesses per second
    let seconds_to_crack = 2f64.powi(entropy_bits as i32) / guesses_per_second;
    seconds_to_crack / (365.25 * 24.0 * 60.0 * 60.0)
}

fn to_checksum_address_into(address: &str, out: &mut String) {
    let hash = Keccak256::digest(address.as_bytes());

    for (i, c) in address.chars().enumerate() {
        if c.is_ascii_digit() {
            out.push(c);
        } else {
            let hash_byte = hash[i / 2];
            let nibble = if i % 2 == 0 {
                hash_byte >> 4
            } else {
                hash_byte & 0x0F
            };
            if nibble >= 8 {
                out.push(c.to_ascii_uppercase());
            } else {
                out.push(c);
            }
        }
    }
}
