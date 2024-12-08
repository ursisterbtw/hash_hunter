use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash_hunter::{to_checksum_address, verify_address};
use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

fn benchmark_address_generation(c: &mut Criterion) {
    c.bench_function("generate_address", |b| {
        let secp = Secp256k1::new();
        let mut rng = OsRng;

        b.iter(|| {
            let secret_key = SecretKey::new(&mut rng);
            let public_key = PublicKey::from_secret_key(&secp, &secret_key);
            black_box(public_key);
        });
    });
}

fn benchmark_checksum(c: &mut Criterion) {
    let test_address = "5aaeb6053f3e94c9b9a09f33669435e7ef1beaed";
    c.bench_function("checksum_address", |b| {
        b.iter(|| {
            black_box(to_checksum_address(black_box(test_address)));
        });
    });
}

criterion_group!(benches, benchmark_address_generation, benchmark_checksum);
criterion_main!(benches);
