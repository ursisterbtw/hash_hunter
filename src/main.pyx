# cython: language_level=3, boundscheck=False, wraparound=False, cdivision=True

from libc.stdlib cimport malloc, free
from libc.string cimport strcpy
from cpython.mem cimport PyMem_Malloc, PyMem_Free

import os
import sys
import json
import argparse
import multiprocessing
from multiprocessing import Value, Lock, Process, Queue, Manager
from time import sleep, time

from eth_account import Account
from web3 import Web3

cdef struct AddressInfo:
    char* address
    char* private_key
    long attempts
    double rarity_score

cdef class VanityAddressGenerator:
    cdef:
        char* start_pattern
        char* end_pattern
        bint use_checksum
        long step
        long max_tries
        long log_interval
        AddressInfo* found_addresses
        long found_count
        long max_found
        object attempts
        object lock
        object found
        object manager
        object result_queue
        list processes
        int num_processes

    def __cinit__(self, start_pattern, end_pattern, use_checksum, step, max_tries, log_interval, num_processes=4):
        self.start_pattern = <char*>PyMem_Malloc((len(start_pattern) + 1) * sizeof(char))
        strcpy(self.start_pattern, start_pattern.encode('utf-8'))
        self.end_pattern = <char*>PyMem_Malloc((len(end_pattern) + 1) * sizeof(char))
        strcpy(self.end_pattern, end_pattern.encode('utf-8'))
        self.use_checksum = use_checksum
        self.step = step
        self.max_tries = max_tries
        self.log_interval = log_interval
        self.max_found = 10
        self.found_addresses = <AddressInfo*>PyMem_Malloc(self.max_found * sizeof(AddressInfo))
        self.found_count = 0
        self.attempts = Value('l', 0)
        self.lock = Lock()
        self.found = Value('i', False)
        self.manager = Manager()
        self.result_queue = self.manager.Queue()
        self.num_processes = num_processes
        self.processes = []

    def __dealloc__(self):
        PyMem_Free(self.start_pattern)
        PyMem_Free(self.end_pattern)
        PyMem_Free(self.found_addresses)

    cpdef generate_address(self):
        cdef:
            str address
            str private_key
            double rarity_score
            long attempts_local

        while self.attempts.value < self.max_tries and not self.found.value:
            account = Account.create()
            private_key = account.key.hex()
            address = account.address.lower()

            if self.use_checksum:
                address = Web3.to_checksum_address(address)

            if (address.startswith(self.start_pattern.decode('utf-8')) and
                address.endswith(self.end_pattern.decode('utf-8'))):
                rarity_score = self.calculate_rarity_score(address)
                self.result_queue.put((address, private_key, self.attempts.value, rarity_score))
                with self.lock:
                    self.found.value = True
                break

            with self.lock:
                self.attempts.value += 1
                attempts_local = self.attempts.value

            if attempts_local % self.step == 0:
                print(f"Checked {attempts_local} addresses")

    cpdef double calculate_rarity_score(self, str address):
        cdef int[16] char_counts
        cdef int i, max_count = 0
        cdef double unique_chars, repetition_factor
        cdef char c

        for i in range(16):
            char_counts[i] = 0

        for i in range(40):
            c = ord(address[i])
            if ord('0') <= c <= ord('9'):
                char_counts[c - ord('0')] += 1
            elif ord('a') <= c <= ord('f'):
                char_counts[c - ord('a') + 10] += 1
            if char_counts[i % 16] > max_count:
                max_count = char_counts[i % 16]

        unique_chars = 0
        for i in range(16):
            if char_counts[i] > 0:
                unique_chars += 1

        repetition_factor = max_count / 40.0

        return (unique_chars / 16.0) * (1.0 + repetition_factor)

    cpdef run(self):
        for _ in range(self.num_processes):
            p = Process(target=self.generate_address)
            self.processes.append(p)
            p.start()

        start_time = time()

        while not self.found.value and self.attempts.value < self.max_tries:
            sleep(self.log_interval / 1000.0)
            with self.lock:
                current_attempts = self.attempts.value

            elapsed = time() - start_time
            print(f"Attempts: {current_attempts} | Time Elapsed: {elapsed:.2f}s")

        for p in self.processes:
            p.join()

        if not self.result_queue.empty():
            result = self.result_queue.get()
            self.save_result(result)

    cpdef save_result(self, tuple result):
        cdef str address, private_key
        cdef long attempts_local
        cdef double rarity_score
        address, private_key, attempts_local, rarity_score = result

        cdef dict output = {
            "address": address,
            "private_key": private_key,
            "attempts": attempts_local,
            "rarity_score": rarity_score
        }

        if not os.path.exists("gen"):
            os.makedirs("gen")

        filename = os.path.join("gen", f"{address}.json")
        with open(filename, "w") as f:
            json.dump(output, f, indent=4)

        print(f"Address found and saved to {filename}")

def main():
    import sys
    parser = argparse.ArgumentParser(description="Vanity Ethereum Address Generator")
    parser.add_argument('-p', '--start-pattern', type=str, default="", help='Prefix of the ETH address')
    parser.add_argument('-e', '--end-pattern', type=str, default="", help='Suffix of the ETH address')
    parser.add_argument('-c', '--checksum', action='store_true', help='Enable EIP-55 checksum')
    parser.add_argument('-s', '--step', type=int, default=50000, help='Number of attempts between progress logs')
    parser.add_argument('-m', '--max-tries', type=int, default=1000000000, help='Maximum number of attempts')
    parser.add_argument('-i', '--log-interval', type=int, default=10000, help='Logging interval in milliseconds')
    parser.add_argument('-n', '--num-processes', type=int, default=4, help='Number of parallel processes')

    args = parser.parse_args()

    generator = VanityAddressGenerator(
        start_pattern=args.start_pattern,
        end_pattern=args.end_pattern,
        use_checksum=args.checksum,
        step=args.step,
        max_tries=args.max_tries,
        log_interval=args.log_interval,
        num_processes=args.num_processes
    )

    generator.run()
