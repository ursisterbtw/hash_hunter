use clap::Parser;
use colored::*;
use dashmap::DashMap;
use num_cpus;
use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// eth addy gen in rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// prefix of the eth address
    #[arg(short = 'p', long, default_value = "6969")]
    start_pattern: String,

    /// suffix of the eth address
    #[arg(short = 'e', long, default_value = "6969")]
    end_pattern: String,

    /// enable EIP-55 checksum
    #[arg(short = 'c', long)]
    checksum: bool,

    /// # of attempts between progress logs
    #[arg(short = 's', long, default_value_t = 50_000)]
    step: u64,

    /// max # of attempts
    #[arg(short = 'm', long, default_value_t = 5_000_000_000_000_000)]
    max_tries: u64,

    /// logging interval in ms
    #[arg(short = 'i', long, default_value_t = 5000)]
    log_interval: u64,
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

    println!("Starting Vanity Address Generator 🚀");
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
    println!("Step: {}", step.to_string().yellow());
    println!("Max Tries: {}", max_tries.to_string().yellow());
    println!("Log Interval (ms): {}", log_interval.to_string().yellow());

    let found = Arc::new(AtomicBool::new(false));
    let result_map = Arc::new(DashMap::new());
    let total_attempts = Arc::new(DashMap::new());
    total_attempts.insert("attempts", 0u64);

    let start_time = Instant::now();

    // start logs
    {
        let total_attempts = Arc::clone(&total_attempts);
        let found = Arc::clone(&found);
        std::thread::spawn(move || {
            while !found.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(log_interval));
                let attempts = total_attempts.get("attempts").map(|a| *a).unwrap_or(0);
                println!(
                    "Total checked addresses: {} 🔍",
                    attempts.to_string().cyan()
                );
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

                    // check prefix and suffix
                    if final_address.starts_with(&start_pattern)
                        && final_address.ends_with(&end_pattern)
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

    // Create 'gen' directory if it doesn't exist
    std::fs::create_dir_all("gen").expect("Failed to create 'gen' directory");

    // check if a result was found
    if let Some(result) = result_map.get("result") {
        println!("\n{}", "🎈 Address found!🎈".bright_green().bold());
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
        let filename = format!("gen/{}.txt", result.address);

        // write to file
        std::fs::write(
            &filename,
            format!(
                "Address: {}\nPrivate Key: {}\nTotal attempts: {}",
                result.address, result.priv_key, result.attempts
            ),
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
        "║                 hash_hunter addy generator                    ║".bright_cyan()
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

/// converts an eth address to its EIP-55 checksummed version
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
