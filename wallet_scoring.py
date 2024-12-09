import json
import os

import matplotlib.pyplot as plt


def calculate_score(wallet_address):
    # ensure the address is 42 characters long, including '0x'
    if len(wallet_address) != 42 or not wallet_address.lower().startswith("0x"):
        raise ValueError(f"Invalid address format: {wallet_address}")

    # remove '0x' prefix
    addr = wallet_address[2:]

    # count total zeroes and leading zeroes
    total_zeroes = addr.count("0")
    leading_zeroes = len(addr) - len(addr.lstrip("0"))

    # calculate score based on total zero count
    # a perfect score would have 41 zeroes (42 chars - 1 for '0x')
    max_zeroes = 41
    score = (total_zeroes / max_zeroes) * 100
    return score, total_zeroes, leading_zeroes


def plot_wallet_scores(wallets):
    if not wallets:
        print("No wallets with leading zeroes to plot.")
        return

    scores = [wallet["score"] for wallet in wallets]
    plt.hist(scores, bins=10, range=(0, 100), alpha=0.75)
    plt.title("Wallet Score Distribution")
    plt.xlabel("Score")
    plt.ylabel("Number of Wallets")
    plt.show()


def run_wallet_check():
    matched_wallets = []
    gen_directory = "gen"

    for filename in os.listdir(gen_directory):
        if filename.endswith(".txt") or filename.endswith(".json"):
            file_path = os.path.join(gen_directory, filename)
            try:
                with open(file_path, "r", encoding="utf-8") as file:
                    if filename.endswith(".json"):
                        # parse JSON file
                        data = json.load(file)
                        if isinstance(data, dict):
                            wallet_addresses = [data.get("address")]
                        elif isinstance(data, list):
                            wallet_addresses = [
                                item.get("address")
                                for item in data
                                if isinstance(item, dict)
                            ]
                        else:
                            print(f"Unexpected JSON structure in {filename}")
                            continue
                    else:
                        # parse TXT file
                        content = file.read().strip()
                        lines = content.split("\n")
                        wallet_addresses = [
                            line.split("Address:")[1].strip()
                            for line in lines
                            if line.startswith("Address:")
                        ]

                    for wallet_address in wallet_addresses:
                        if wallet_address:
                            try:
                                score, total_zeroes, leading_zeroes = calculate_score(
                                    wallet_address
                                )
                                print(
                                    f"Wallet: {wallet_address}, "
                                    f"Total Zeroes: {total_zeroes}, "
                                    f"Leading Zeroes: {leading_zeroes}, "
                                    f"Score: {score:.2f}%"
                                )
                                matched_wallets.append(
                                    {"address": wallet_address, "score": score}
                                )
                            except ValueError as e:
                                print(f"Error processing address: {e}")

            except (UnicodeDecodeError, json.JSONDecodeError) as e:
                print(f"Error reading file {filename}: {e}")

    plot_wallet_scores(matched_wallets)


run_wallet_check()
