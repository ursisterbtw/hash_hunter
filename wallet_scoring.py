import os
import matplotlib.pyplot as plt


def calculate_score(wallet_address):
    # ensure the address is 42 characters long, including '0x'
    if len(wallet_address) != 42 or not wallet_address.lower().startswith("0x"):
        raise ValueError(f"Invalid address format: {wallet_address}")

    # remove '0x' prefix
    addr = wallet_address[2:]

    # count the number of leading zeroes
    leading_zeroes = len(addr) - len(addr.lstrip("0"))

    # count total zeroes
    total_zeroes = addr.count("0")

    # calculate score based on leading zero count
    # a perfect score would have 40 leading zeroes (42 chars - 2 for '0x')
    max_leading_zeroes = 40
    score = (leading_zeroes / max_leading_zeroes) * 100
    return score, leading_zeroes, total_zeroes


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

                                # only include wallets with at least one leading zero
                                if leading_zeroes > 0:
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
