use clap::Parser;
use colored::*;
use dashmap::DashMap;
use indicatif::{ProgressBar, ProgressStyle};
use num_cpus;
use rand::rngs::OsRng;
use regex::Regex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// eth addy gen in rust, zooms
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // prefix of the eth address
    #[arg(short = 'p', long, default_value = "")]
    start_pattern: String,

    // suffix of the eth address
    #[arg(short = 'e', long, default_value = "69")]
    end_pattern: String,

    // enable EIP-55 checksum
    #[arg(short = 'c', long, default_value_t = true)]
    checksum: bool,

    // # of attempts between progress logs
    #[arg(short = 's', long, default_value_t = 50_000)]
    step: u64,

    // max # of attempts
    #[arg(short = 'm', long, default_value_t = 1_000_000_000)]
    max_tries: u64,

    // logging interval in ms
    #[arg(short = 'i', long, default_value_t = 10_000)]
    log_interval: u64,

    // minimum number of zeros in the address
    #[arg(short = 'z', long, default_value_t = 0)]
    min_zeros: usize,

    // regex pattern to match in the address
    #[arg(short = 'r', long, default_value = "DEADBEEF")]
    regex_pattern: String,
}

struct VanityResult {
    address: String,
    priv_key: String,
    attempts: u64,
}

fn main() {
    print_startup_screen();

    let args = Args::parse();

    let start_pattern = args.start_pattern.to_lowercase();
    let end_pattern = args.end_pattern.to_lowercase();
    let use_checksum = args.checksum;
    let step = args.step;
    let max_tries = args.max_tries;
    let log_interval = args.log_interval;
    let min_zeros = args.min_zeros;

    let regex_pattern = if !args.regex_pattern.is_empty() {
        Some(Arc::new(
            Regex::new(&args.regex_pattern).expect("Invalid regex pattern"),
        ))
    } else {
        None
    };

    // add a confirmation prompt
    if !confirm_start(&args) {
        println!("Operation cancelled by user.");
        return;
    }

    println!("Starting Vanity Address Generator 🧪");
    println!("Prefix: {}", args.start_pattern.bright_green());
    println!("Suffix: {}", args.end_pattern.bright_green());
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
            while !found.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(log_interval));
                let attempts = total_attempts.get("attempts").map(|a| *a).unwrap_or(0);
                progress_bar.set_position(attempts);
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
            let use_checksum = use_checksum;
            let step = step;
            let max_tries = max_tries;
            let found = Arc::clone(&found);
            let result_map = Arc::clone(&result_map);
            let total_attempts = Arc::clone(&total_attempts);
            let min_zeros = min_zeros;
            let regex_pattern = regex_pattern.clone(); // Clone the regex pattern for each thread

            s.spawn(move |_| {
                let secp = Secp256k1::new();
                let mut rng = OsRng;

                let mut local_attempts = 0u64;

                while !found.load(Ordering::Relaxed)
                    && total_attempts.get("attempts").map_or(0, |a| *a) < max_tries
                {
                    // key gen
                    let secret_key = SecretKey::new(&mut rng);

                    // compute pubkey
                    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
                    let serialized_pub = public_key.serialize_uncompressed();

                    // compute the keccak-256 hash of the public key ( - first byte)
                    let hash = Keccak256::digest(&serialized_pub[1..]);

                    // take last 20 bytes as address
                    let address = hex::encode(&hash[12..]);

                    // apply checksum if enabled
                    let final_address = if use_checksum {
                        to_checksum_address(&address)
                    } else {
                        address.clone()
                    };

                    // check prefix, suffix, minimum zeros, and regex pattern
                    let zero_count = final_address.matches('0').count();
                    if final_address.starts_with(&start_pattern)
                        && final_address.ends_with(&end_pattern)
                        && zero_count >= min_zeros
                        && regex_pattern
                            .as_ref()
                            .map_or(true, |re| re.is_match(&final_address))
                    {
                        // found a matching address
                        let address_with_prefix = format!("0x{}", final_address);
                        let priv_key_hex = hex::encode(secret_key.as_ref());

                        // insert the result
                        result_map.insert(
                            "result",
                            VanityResult {
                                address: address_with_prefix.clone(),
                                priv_key: priv_key_hex.clone(),
                                attempts: total_attempts.get("attempts").map(|a| *a).unwrap_or(0),
                            },
                        );

                        // signal other threads to stop
                        found.store(true, Ordering::Relaxed);
                        break;
                    }

                    // increment counters
                    local_attempts += 1;
                    if local_attempts >= step {
                        total_attempts
                            .entry("attempts")
                            .and_modify(|a| *a += local_attempts);
                        local_attempts = 0;
                    }
                }

                // add remaining attempts
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
        println!("\n{}", "🌀 Address found!🌀".bright_green().bold());
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

        // write to file
        std::fs::write(
            &filename,
            serde_json::to_string_pretty(&json_output).unwrap(),
        )
        .expect("Unable to write to file");

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
    let secret_key = SecretKey::from_slice(&hex::decode(private_key).unwrap()).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let serialized_pub = public_key.serialize_uncompressed();
    let hash = Keccak256::digest(&serialized_pub[1..]);
    let generated_address = format!("0x{}", hex::encode(&hash[12..]));

    address.to_lowercase() == generated_address.to_lowercase()
}

// converts an eth address to its EIP-55 checksummed version
fn to_checksum_address(address: &str) -> String {
    let address = address.to_lowercase();
    let hash = Keccak256::digest(address.as_bytes());
    let mut checksum_address = String::with_capacity(40);

    for (i, c) in address.chars().enumerate() {
        if c.is_digit(10) {
            checksum_address.push(c);
        } else {
            let hash_byte = hash[i / 2];
            let nibble = if i % 2 == 0 {
                hash_byte >> 4
            } else {
                hash_byte & 0x0F
            };
            if nibble >= 8 {
                checksum_address.push(c.to_ascii_uppercase());
            } else {
                checksum_address.push(c);
            }
        }
    }
    // ggez
    checksum_address
}

fn confirm_start(_args: &Args) -> bool {
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
