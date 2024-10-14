# cython: language_level=3

from libc.stdlib cimport malloc, free, realloc
from libc.string cimport memcpy, strlen, strcpy, memcmp
from libc.stdio cimport printf


import os
import sys
import numpy as np
cimport numpy as np

from eth_account import Account
from eth_account.messages import encode_defunct
from web3 import Web3

cdef extern from "Python.h":
    const char* PyUnicode_AsUTF8(object unicode)

cdef struct AddressInfo:
    char* address
    char* private_key
    int attempts
    double rarity_score

cdef class VanityAddressGenerator:
    cdef:
        const char* start_pattern
        const char* end_pattern
        bint use_checksum
        int step
        int log_interval
        AddressInfo* found_addresses
        int found_count
        int max_found

    def __cinit__(self, str start_pattern, str end_pattern, bint use_checksum, int step, int log_interval):
        self.start_pattern = PyUnicode_AsUTF8(start_pattern)
        self.end_pattern = PyUnicode_AsUTF8(end_pattern)
        self.use_checksum = use_checksum
        self.step = step
        self.log_interval = log_interval
        self.max_found = 10
        self.found_addresses = <AddressInfo*>malloc(self.max_found * sizeof(AddressInfo))
        self.found_count = 0

    def __dealloc__(self):
        for i in range(self.found_count):
            free(self.found_addresses[i].address)
            free(self.found_addresses[i].private_key)
        free(self.found_addresses)

    cdef bint is_palindrome(self, const char* s):
        cdef int i, j
        cdef int length = strlen(s)
        for i in range(length // 2):
            if s[i] != s[length - 1 - i]:
                return False
        return True

    cdef double calculate_rarity_score(self, const char* address):
        cdef int char_counts[16]
        cdef int i, max_count = 0
        cdef double unique_chars, repetition_factor

        for i in range(16):
            char_counts[i] = 0

        for i in range(40):
            if b'0'[0] <= address[i] <= b'9'[0]:
                char_counts[address[i] - b'0'[0]] += 1
            elif b'a'[0] <= address[i] <= b'f'[0]:
                char_counts[address[i] - b'a'[0] + 10] += 1
            if char_counts[i] > max_count:
                max_count = char_counts[i]

        unique_chars = 0
        for i in range(16):
            if char_counts[i] > 0:
                unique_chars += 1

        repetition_factor = max_count / 40.0

        return (unique_chars / 16.0) * (1.0 + repetition_factor)

    cdef void generate_address(self):
        cdef const char* address
        cdef const char* private_key
        cdef int attempts = 0
        cdef double rarity_score

        while True:
            account = Account.create()
            private_key = PyUnicode_AsUTF8(account._private_key.hex())
            address = PyUnicode_AsUTF8(account.address.lower())

            if self.use_checksum:
                address = PyUnicode_AsUTF8(Web3.to_checksum_address(address))

            if (memcmp(address, self.start_pattern, strlen(self.start_pattern)) == 0 and
                memcmp(address + strlen(address) - strlen(self.end_pattern), self.end_pattern, strlen(self.end_pattern)) == 0) or \
               self.is_palindrome(address + 2):
                rarity_score = self.calculate_rarity_score(address + 2)
                self.add_found_address(address, private_key, attempts, rarity_score)
                break

            attempts += 1

            if attempts % self.step == 0:
                printf("Checked %d addresses\n", attempts)

    cdef void add_found_address(self, const char* address, const char* private_key, int attempts, double rarity_score):
        if self.found_count >= self.max_found:
            self.max_found *= 2
            self.found_addresses = <AddressInfo*>realloc(self.found_addresses, self.max_found * sizeof(AddressInfo))

        cdef AddressInfo* info = &self.found_addresses[self.found_count]
        info.address = <char*>malloc((strlen(address) + 1) * sizeof(char))
        info.private_key = <char*>malloc((strlen(private_key) + 1) * sizeof(char))
        strcpy(info.address, address)
        strcpy(info.private_key, private_key)
        info.attempts = attempts
        info.rarity_score = rarity_score
        self.found_count += 1

    def run(self):
        self.generate_address()

    def get_found_addresses(self):
        return [(self.found_addresses[i].address.decode('utf-8'),
                 self.found_addresses[i].private_key.decode('utf-8'),
                 self.found_addresses[i].attempts,
                 self.found_addresses[i].rarity_score)
                for i in range(self.found_count)]

def main():
    generator = VanityAddressGenerator("", "6969", True, 50000, 5000)
    generator.run()
    
    for address, private_key, attempts, rarity_score in generator.get_found_addresses():
        print(f"Found address: {address}")
        print(f"Private key: {private_key}")
        print(f"Attempts: {attempts}")
        print(f"Rarity score: {rarity_score:.4f}")

        filename = f"gen/{address}.txt"
        with open(filename, "w") as f:
            f.write(f"Address: {address}\n")
            f.write(f"Private Key: {private_key}\n")
            f.write(f"Attempts: {attempts}\n")
            f.write(f"Rarity Score: {rarity_score:.4f}\n")

        print(f"Wallet information saved to {filename}")

if __name__ == "__main__":
    main()