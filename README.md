# parity-cli

A password generator using Parity Brain Wallets wordlist library.

The cli takes an argument of the number of words to generate. If you just give it a number, then it generates a passphrase with the same number of words (spaces between).

The cli also takes an option (short: -p, long: --password) that generates the number of words provided, connected with hyphens, and adds a random number between 0 and 9999999 to the end. The password option also randomly capitalizes roughly half of the characters generated from the wordlist.

Example usage:
```bash
# generate a password with 4 words, random lowercase and capital letters, and a random integer
cargo run -- 4 -p

# generate a 15 word passphrase
cargo run -- 15
```

The best part is some of the word combinations are funny.

