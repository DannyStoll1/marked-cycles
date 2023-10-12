A tool for computing the combinatorics of marked cycle curves over dynamical moduli spaces.

## Installing and Running

0. Ensure that you have Rust installed. You can install Rust via e.g. [Rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

1. Compile the repository: `cargo build --release && cd target/release`

2. Run the binary, e.g. `./marked-cycles --crit-period 2 --marked-period 6`

## Options

The following command line arguments can be passed to `marked-cycles`:

  -m, --marked-period <MARKED_PERIOD>
          Period of the marked cycle (0 to skip) [default: 0]
  -c, --crit-period <CRIT_PERIOD>
          Period of the critical cycle (must be 1 or 2 for now) [default: 1]
  -t, --table-max-period <TABLE_MAX_PERIOD>
          Max period of data table (0 to skip) [default: 0]
  -d, --dynatomic
          Compute dynatomic curve instead of marked cycle curve
  -b, --binary
          Display cell ids in binary
      --indent <INDENT>
          How far to indent the cell descriptions [default: 4]
  -h, --help
          Print help

These options are also available via `./marked-cycles --help`.
