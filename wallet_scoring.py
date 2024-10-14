import os
import matplotlib.pyplot as plt


def calculate_score(wallet_address):
    # ensure the address is 42 characters long, including '0x'
    if len(wallet_address) != 42 or not wallet_address.startswith("0x"):
        raise ValueError(f"Invalid address format: {wallet_address}")

    # count the number of leading zeroes (including '0x')
    leading_zeroes = len(wallet_address) - len(wallet_address.lstrip("0x0"))

    # count total zeroes
    total_zeroes = wallet_address.count("0")

    # calculate score based on leading zero count
    # a perfect score would have 40 leading zeroes (42 chars - 2 non-zero chars)
    max_leading_zeroes = 40
    score = (leading_zeroes / max_leading_zeroes) * 100
    return score, leading_zeroes, total_zeroes


def plot_wallet_scores(wallets):
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
        if filename.endswith(".txt"):
            file_path = os.path.join(gen_directory, filename)
            try:
                with open(file_path, "r", encoding="utf-8") as file:
                    content = file.read().strip()
                    lines = content.split("\n")
                    for line in lines:
                        if line.startswith("Address:"):
                            wallet_address = line.split("Address:")[1].strip()

                            try:
                                score, leading_zeroes, total_zeroes = calculate_score(
                                    wallet_address
                                )
                                print(
                                    f"Wallet: {wallet_address}, Leading Zeroes: {leading_zeroes}, "
                                    f"Total Zeroes: {total_zeroes}, Score: {score:.2f}%"
                                )
                                matched_wallets.append(
                                    {"address": wallet_address, "score": score}
                                )
                            except ValueError as e:
                                print(f"Error processing address: {e}")

            except UnicodeDecodeError:
                print(f"Skipping file {filename} due to encoding issues")

    plot_wallet_scores(matched_wallets)


run_wallet_check()
