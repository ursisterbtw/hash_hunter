use hash_hunter::{calculate_years_to_crack, to_checksum_address, verify_address, VanityResult};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

#[test]
fn test_checksum_address() {
    let test_address = "5aaeb6053f3e94c9b9a09f33669435e7ef1beaed";
    let expected = "5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeD";
    assert_eq!(to_checksum_address(test_address), expected);
}

#[test]
fn test_verify_address() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&[0x01; 32]).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let serialized_pub = public_key.serialize_uncompressed();
    let hash = Keccak256::digest(&serialized_pub[1..]);
    let address = format!("0x{}", hex::encode(&hash[12..]));
    let private_key = hex::encode(secret_key.as_ref());

    assert!(verify_address(&address, &private_key));
}

#[test]
fn test_entropy_calculation() {
    let test_cases = vec![
        ("0x1234567890abcdef", 60.0),
        ("0x0000000000000000", 60.0),
        ("0xffffffffffffffff", 60.0),
    ];

    for (address, expected_bits) in test_cases {
        let address_without_prefix = address.trim_start_matches("0x");
        let entropy_bits = address_without_prefix.len() * 4;
        assert_eq!(entropy_bits as f64, expected_bits);
    }
}

#[test]
fn test_years_to_crack() {
    let test_cases = vec![
        (128, 1.0e20), // 128-bit should take very long
        (64, 1.0e8),   // 64-bit should be significantly less
        (32, 1.0),     // 32-bit should be relatively quick
    ];

    for (bits, min_years) in test_cases {
        let years = calculate_years_to_crack(bits);
        assert!(
            years > min_years,
            "Expected more than {} years for {}-bit entropy, got {}",
            min_years,
            bits,
            years
        );
    }
}

#[test]
fn test_pattern_matching() {
    let test_cases = vec![
        ("0x1234", "4321", true),
        ("0xabcd", "dcba", true),
        ("0x0000", "0000", true),
        ("0x1234", "5678", false),
    ];

    for (start, end, should_match) in test_cases {
        let address = format!("{}middle{}", start, end);
        assert_eq!(
            address.starts_with(start) && address.ends_with(end),
            should_match,
            "Pattern matching failed for {}",
            address
        );
    }
}
