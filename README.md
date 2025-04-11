[![actions status][actions-badge]][actions-url]
[![crate version][crates-version-badge]][crates-url]
[![documentation][docs-badge]][docs-url]
[![dependencies status][deps-badge]][deps-url]
![licenses][licenses-badge]

[actions-badge]: https://github.com/cecton/libc-system/workflows/Rust/badge.svg
[actions-url]: https://github.com/cecton/libc-system/actions
[crates-version-badge]: https://img.shields.io/crates/v/libc-system
[crates-url]: https://crates.io/crates/libc-system
[docs-badge]: https://docs.rs/libc-system/badge.svg
[docs-url]: https://docs.rs/libc-system/
[deps-badge]: https://deps.rs/repo/github/cecton/libc-system/status.svg
[deps-url]: https://deps.rs/crate/libc-system
[licenses-badge]: https://img.shields.io/crates/l/libc-system

<!-- cargo-rdme start -->

# libc-system

**libc-system** provides a single cross‑platform function, [`system`], to execute a shell
command and return its exit code.

## Overview

- On **non‑Windows** systems, it calls the C library’s `system()` function. The command is
  passed as a narrow (UTF‑8) null‑terminated C string.
- On **Windows**, it calls the C runtime’s `_wsystem()` function. The command is converted from
  UTF‑8 to a wide (UTF‑16) string to properly handle non‑ASCII characters.

This crate lets you execute shell commands without having to worry about platform-specific
differences.

## Example

```rust
use libc_system::system;

let exit_code = system("echo Hello, World!");
println!("Exit code: {}", exit_code);
```

## Platform-specific details

- **Windows:** Uses `_wsystem` from the C runtime. The command is converted into a wide
  (UTF‑16) string.
- **Others:** Uses `system` from libc. The command is converted into a narrow, null‑terminated
  string.

<!-- cargo-rdme end -->
