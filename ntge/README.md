# Not That Good Encryption (NTGE)

A Command-Line-Interface tool that wraps the [ntge-core](https://crates.io/crates/ntge-core) API.

## Get Started

`$ cargo build`

Build the project first

`$ cargo run --example create`

It creates a new keypair `example_key` and saves it at `~/.ntge/`

`$ cargo run --example encrypt`

It encrypts the message `Welcome to use NTGE!` to `example_key` and prints the ciphertext payload on the terminal.

`$ cargo run --example encrypt_sign`

It encrypts the message `Welcome to use NTGE!` to `example_key`, signs it as `example_key` and prints the ciphertext payload on the terminal.

`$ cargo run --example decrypt`

It encrypts the message `Decrypt Succeeds! Welcome to use NTGE!` to `example_key`, decrypt the ciphertext with `example_key` and prints the plaintext on the terminal.

`$ cargo run --example verify`

It encrypts the message `Decrypt Succeeds! Welcome to use NTGE!` to `example_key`, verify the signature in the payload with `example_key.pub` and prints `message signature verified by example_key` on the terminal.
