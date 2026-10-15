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

| Crate | Version | crates.io | Purpose |
|-------|---------|-----------|---------|
| `vdf-rs` | 0.2.1 | yes | VDF trait and Pietrzak / Wesolowski implementations |
| `vdf-classgroup` | 0.2.1 | yes | Class group arithmetic (GMP-backed) |
| `vdf-rs-cli` | 0.2.1 | yes | CLI binary `vdf-cli` (`cargo install vdf-rs-cli`; name `vdf-cli` is taken on crates.io) |
| `vdf-competition` | — | no | Competition helper binary |
| `generate-discriminants` | — | no | Internal discriminant table helper |

## Requirements

- **MSRV:** [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) 1.85+ (`rust-version = "1.85"`)
- **GNU Multiple Precision Library (GMP)** — required at **build and link** time. `vdf-classgroup` (and therefore `vdf-rs` / `vdf-cli`) is not a pure-Rust crate. Install `libgmp` before `cargo build`, `cargo test`, or `cargo publish`.

On Debian/Ubuntu:

```sh
sudo apt-get install -y libgmp-dev
```

On macOS (Homebrew):

```sh
brew install gmp
```

The arithmetic backend is a vendored `rust-gmp` copy. Known FFI limitations
(in-place `mpz` layout, two’s-complement export, duplicate `extern` prototypes)
are deferred to **0.3.0**. This 0.2.1 release does **not** change the GMP backend.

## Usage

From crates.io:

```sh
cargo install vdf-rs-cli
```

From a git checkout:

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
cargo test --workspace -- --nocapture
./bench.sh aadf
```

See [CHANGELOG.md](https://github.com/jose-compu/vdf-rs/blob/main/CHANGELOG.md) for release notes.

## License

Copyright 2018 Chia Network Inc and POA Networks Ltd.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
