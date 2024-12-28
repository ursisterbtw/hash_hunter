use actix_web::{web, App, HttpServer, Responder};
use ethers::prelude::*;
use ethers::utils::hex;
use indicatif::{ProgressBar, ProgressStyle};
use rand::thread_rng;
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    sync::Arc,
};
use tokio::sync::Mutex;

struct SearchStats {
    attempts: u64,
    start_time: std::time::Instant,
}

async fn generate_salted_address(stats: web::Data<Arc<Mutex<SearchStats>>>) -> impl Responder {
    let target_zeros = 10;
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );

    loop {
        let wallet = LocalWallet::new(&mut thread_rng());
        let address = wallet.address();
        let address_str = format!("{:x}", address);

        let mut stats = stats.lock().await;
        stats.attempts += 1;

        // update progress every 10000 attempts        if stats.attempts % 10000 == 0 {
            let rate = stats.attempts as f64 / stats.start_time.elapsed().as_secs_f64();
            pb.set_message(format!(
                "Attempts: {} | Rate: {:.0} addr/s | Elapsed: {:?}",
                stats.attempts,
                rate,
                stats.start_time.elapsed()
            ));
        }

        let leading_zeros = address_str
            .chars()
            .skip(2)
            .take_while(|&c| c == '0')
            .count();

        if leading_zeros >= target_zeros {
            let private_key = wallet.signer().to_bytes().to_vec();

            // create results directory if it doesn't exist            fs::create_dir_all("results").unwrap_or_default();

            // save to file            let filename = format!("results/{}.txt", address);
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(&filename)
                .unwrap();

            let result = format!(
                "Address: {}\nPrivate Key: 0x{}\nAttempts: {}\nTime: {:?}\nRate: {:.0} addr/s\n",
                address,
                hex::encode(&private_key),
                stats.attempts,
                stats.start_time.elapsed(),
                stats.attempts as f64 / stats.start_time.elapsed().as_secs_f64()
            );

            file.write_all(result.as_bytes()).unwrap();

            pb.finish_with_message(format!(
                "🎯 Found after {} attempts!\n📫 Address: {}\n🔑 Private Key: 0x{}\n💾 Saved to {}\n",
                stats.attempts,
                address,
                hex::encode(&private_key),
                filename
            ));

            return result;
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "6969".to_string());

    let stats = Arc::new(Mutex::new(SearchStats {
        attempts: 0,
        start_time: std::time::Instant::now(),
    }));

    println!("🚀 Starting server on port {}", port);
    println!("🎯 Searching for addresses with 10+ leading zeros...\n");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(stats.clone()))
            .route("/salt", web::get().to(generate_salted_address))
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}
