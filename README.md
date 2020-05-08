# Not That Good Encryption (NTGE)

![Rust](https://github.com/DimensionDev/ntge/workflows/Rust/badge.svg)

Not That Good Encryption is a general-purpose rust-based encryption tool. Our main goal is to learn rust in a project-based way for a real on-hand practice for the team. We are also willing to integrate this tool/lib to our new encryption input method mobile app. If our implementation works well enough, we will definitely give it a try.

## ntge
Using Cargo install the ntge. You can checkout this app in [crates.io](https://crates.io/crates/ntge).

```bash
$ cargo install ntge
```

## ntge-core
The underlying core for ntge. Please checks the implementation detail under [ntge-core](./ntge-core).

And ntge-core mobiles SDK povided in the Kotlin & Swift.

- [NtgeCore-Android](./NtgeCore-Android/README.md)
- [NtgeCore-iOS](./NtgeCore-iOS/README.md)

## Examples

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

## Acknowledgements
- [rage](https://github.com/str4d/rage)
