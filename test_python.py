import pytest
from eth_account import Account
import re
from src.main import (
    calculate_rarity_score,
    is_palindrome,
    to_checksum_address,
    verify_address,
)

def test_rarity_score():
    test_cases = [
        ("0x0000000000000000", 0.125),  # All zeros should have low score
        ("0x1234567890abcdef", 1.0),    # All different chars should have high score
        ("0xaaaaaaaaaaaaaaaa", 0.125),  # All same chars should have low score
    ]
    
    for address, expected_score in test_cases:
        score = calculate_rarity_score(address[2:])  # Remove 0x prefix
        assert abs(score - expected_score) < 0.01, f"Failed for {address}"

def test_palindrome_check():
    test_cases = [
        ("1221", True),
        ("abba", True),
        ("1234", False),
        ("12345", False),
    ]
    
    for test_str, expected in test_cases:
        assert is_palindrome(test_str) == expected, f"Failed for {test_str}"

def test_checksum_address():
    test_cases = [
        ("0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed",
         "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeD"),
        ("0x0000000000000000000000000000000000000000",
         "0x0000000000000000000000000000000000000000"),
    ]
    
    for input_addr, expected in test_cases:
        result = to_checksum_address(input_addr)
        assert result == expected, f"Failed for {input_addr}"

def test_address_verification():
    # create a test account
    account = Account.create()
    address = account.address
    private_key = account._private_key.hex()
    
    assert verify_address(address, private_key), "Address verification failed"

def test_pattern_matching():
    patterns = [
        (r"^0x[0-9a-fA-F]{4}0000[0-9a-fA-F]{36}$", "0x12340000abcdef"),
        (r"^0x[0-9a-fA-F]*(1234|2345|3456|4567|5678|6789)[0-9a-fA-F]*$", "0x1234abcdef"),
        (r"^0x[0-9a-fA-F]*(DEADBEEF|BADDCAFE|1337BEEF)[0-9a-fA-F]*$", "0xDEADBEEF"),
    ]
    
    for pattern, test_str in patterns:
        assert re.match(pattern, test_str), f"Pattern {pattern} failed to match {test_str}"

@pytest.mark.parametrize("num_attempts", [1000, 5000, 10000])
def test_address_generation_performance(benchmark, num_attempts):
    def generate_addresses():
        for _ in range(num_attempts):
            Account.create()
    
    result = benchmark(generate_addresses)
    assert result.stats.total_time < num_attempts * 0.1  # Expected time per address

def test_concurrent_generation():
    from concurrent.futures import ThreadPoolExecutor
    import threading
    
    found = threading.Event()
    addresses = []
    
    def generate_address():
        if not found.is_set():
            account = Account.create()
            addresses.append(account.address)
            found.set()
    
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = [executor.submit(generate_address) for _ in range(4)]
        for future in futures:
            future.result()
    
    assert len(addresses) > 0, "No addresses were generated"