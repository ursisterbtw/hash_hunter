import os
import re


def lint_and_uncapitalize_comments(file_path):
    """
    Lint a file for comments with leading capital letters and uncapitalize them.
    Modify the file in place.
    """
    try:
        with open(file_path, "r", encoding="utf-8") as file:
            lines = file.readlines()
        modified_lines = []
        for line in lines:
            stripped_line = line.lstrip()
            if stripped_line.startswith("#"):
                # find the comment text after the `#`
                comment = stripped_line[1:].lstrip()
                # check if it starts with a capital letter, but ignore "SAFETY:" comments
                if (
                    comment
                    and comment[0].isupper()
                    and not comment.lstrip().startswith("SAFETY:")
                ):
                    # replace the line with the uncapitalized comment
                    new_comment = comment[0].lower() + comment[1:]
                    line = line.replace(comment, new_comment, 1)
            modified_lines.append(line)
        # write the changes back to the file
        with open(file_path, "w", encoding="utf-8") as file:
            file.writelines(modified_lines)
        print(f"Processed: {file_path}")
    except Exception as e:
        print(f"Error processing {file_path}: {e}")


def lowercase_comment_leading_letter(file_path):
    try:
        with open(file_path, "r", encoding="utf-8") as file:
            lines = file.readlines()

        with open(file_path, "w", encoding="utf-8") as file:
            for line in lines:
                # skip lowercasing if the comment contains "SAFETY:"
                if "SAFETY:" in line:
                    file.write(line)
                    continue
                match = re.match(r"^\s*//+\s*([A-Z])", line)
                if match:
                    line = re.sub(
                        r"^(\s*//+\s*)([A-Z])",
                        lambda m: m.group(1) + m.group(2).lower(),
                        line,
                    )
                file.write(line)
        print(f"Processed: {file_path}")
    except Exception as e:
        print(f"Error processing {file_path}: {e}")


def process_directory(directory):
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs"):
                file_path = os.path.join(root, file)
                lowercase_comment_leading_letter(file_path)
            elif file.endswith(".py"):  # only process Python files                file_path = os.path.join(root, file)
                lint_and_uncapitalize_comments(file_path)


if __name__ == "__main__":
    dir_path = input("Enter the directory path to process: ").strip()
    if os.path.exists(dir_path):
        process_directory(dir_path)
    else:
        print("Invalid directory path.")
