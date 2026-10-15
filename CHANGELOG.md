# Changelog

## 0.2.1 — 2026-10-15

### Added

- CLI published as **`vdf-rs-cli`** (binary remains `vdf-cli`). The crate name `vdf-cli` is taken on crates.io (unrelated 0.1.0 from 2019).
- Workspace `CHANGELOG.md`.

### Changed

- `vdf-rs`, `vdf-classgroup`, and `vdf-rs-cli` are versioned **0.2.1**.
- README crate table lists `vdf-rs-cli` (`cargo install vdf-rs-cli`; binary `vdf-cli`).
- Documented the GNU MP (`libgmp`) build requirement. The GMP backend is unchanged in 0.2.x; replacing the vendored `rust-gmp` copy remains 0.3.0.

### Fixed

- Discriminant generation uses `AddAssign`/`SubAssign<u64>` (rust-gmp has no i32/i64 `+=`/`-=`); Fouque–Tibouchi is eprint 2011/481 (2011/401 was a mis-citation).

### Deferred to 0.3.0

- Replace the vendored `rust-gmp` GMP backend (`mpz_struct` layout cast, two’s-complement export, duplicate FFI prototypes).
- Remaining `gmp/mpz.rs` TODOs (limb type, radix formatting, zero-modulo, `perfect_root`).

MSRV: Rust 1.85.
