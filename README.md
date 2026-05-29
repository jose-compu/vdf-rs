# vdf-rs — Verifiable Delay Function (VDF) in Rust

Maintained fork of [poanetwork/vdf](https://github.com/poanetwork/vdf). The original
[`vdf`](https://crates.io/crates/vdf) crate on crates.io is unmaintained; this project
publishes the library as **`vdf-rs`**.

## What is a VDF?

A Verifiable Delay Function (VDF) is a function that requires substantial time
to evaluate (even with a polynomial number of parallel processors) but can be
very quickly verified as correct. VDFs can be used to construct randomness
beacons with multiple applications in a distributed network environment. By
introducing a time delay during evaluation, VDFs prevent malicious actors from
influencing output. The output cannot be differentiated from a random number
until the final result is computed. See <https://eprint.iacr.org/2018/712.pdf>
for more details.

## Description

This VDF implementation is written in Rust. The GMP library is used for
arithmetic and greatest common divisor (GCD) calculations. We use class groups
to implement the approaches described in the following papers:

1. [Simple Verifiable Delay Functions](https://eprint.iacr.org/2018/627.pdf). Pietrzak, 2018
2. [Efficient Verifiable Delay Functions](https://eprint.iacr.org/2018/623.pdf). Wesolowski, 2018

The chosen generator is (2, 1, c), where c is calculated from the provided
discriminant. A form is represented internally (a, b, c), with the
discriminant not being used in most computations. This implementation performs
reduction after every multiplication and squaring, as not doing so did not give
any gains in our benchmarks.

## Crates

| Crate | crates.io | Purpose |
|-------|-----------|---------|
| `vdf-rs` | yes | VDF trait and Pietrzak / Wesolowski implementations |
| `vdf-classgroup` | yes | Class group arithmetic (GMP-backed) |
| `vdf-cli` | no | Command-line tool |
| `vdf-competition` | no | Competition helper binary |

## Requirements

- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) 1.85+
- [GNU Multiple Precision Library (GMP)](https://gmplib.org/)

On Debian/Ubuntu:

```sh
sudo apt-get install -y libgmp-dev
```

On macOS (Homebrew):

```sh
brew install gmp
```

## Usage

```sh
git clone https://github.com/jose-compu/vdf-rs.git
cd vdf-rs
cargo install --path vdf-cli
```

### Command-line interface

```sh
vdf-cli aa 100
```

For Pietrzak proofs, pass `-tpietrzak`. Run `vdf-cli --help` for details.

### Library

Add to `Cargo.toml`:

```toml
vdf-rs = "0.2"
```

```rust
use vdf_rs::{PietrzakVDFParams, VDFParams, VDF};

fn main() {
    let vdf = PietrzakVDFParams(2048).new();
    let proof = vdf.solve(b"\xaa", 100).unwrap();
    assert!(vdf.verify(b"\xaa", 100, &proof).is_ok());
}
```

## Development

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
./bench.sh aadf
```

## License

Copyright 2018 Chia Network Inc and POA Networks Ltd.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
