A tool for computing the combinatorics of marked cycle curves over dynamical moduli spaces.

## Installing and Running

0. Ensure that you have Rust installed. You can install Rust via e.g. [Rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

1. Compile the repository: `cargo build --release`

2. Run the binary, e.g. `./target/release/marked-cycles --crit-period 2 --marked-period 6`
