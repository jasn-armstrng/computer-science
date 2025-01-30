# Create a .txt file, 1 KB in size, by writing a single 8 byte char, 1024 times to a file.
if __name__ == "__main__":
    with open("1KB.txt", "w") as file:
        for _ in range(0, 1024):
            file.write("1")

# How does the OS compute the size of a .txt file?
# How does it count ascii and non-ascii?
# Is there a different sizing process for different file types?
