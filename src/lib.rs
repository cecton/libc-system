//! # libc-system
//!
//! **libc-system** provides a single cross‑platform function, [`system`], to execute a shell
//! command and return its exit code.
//!
//! ## Overview
//!
//! - On **non‑Windows** systems, it calls the C library’s `system()` function. The command is
//!   passed as a narrow (UTF‑8) null‑terminated C string.
//! - On **Windows**, it calls the C runtime’s `_wsystem()` function. The command is converted from
//!   UTF‑8 to a wide (UTF‑16) string to properly handle non‑ASCII characters.
//!
//! This crate lets you execute shell commands without having to worry about platform-specific
//! differences.
//!
//! ## Example
//!
//! ```rust
//! use libc_system::system;
//!
//! let exit_code = system("echo Hello, World!");
//! println!("Exit code: {}", exit_code);
//! ```
//!
//! ## Platform-specific details
//!
//! - **Windows:** Uses `_wsystem` from the C runtime. The command is converted into a wide
//!   (UTF‑16) string.
//! - **Others:** Uses `system` from libc. The command is converted into a narrow, null‑terminated
//!   string.

#[cfg(not(windows))]
pub fn system(command: &str) -> i32 {
    extern "C" {
        pub fn system(s: *const std::ffi::c_char) -> std::ffi::c_int;
    }

    let Ok(c_command) = std::ffi::CString::new(command) else {
        return -1;
    };
    unsafe { system(c_command.as_ptr()) }
}

#[cfg(windows)]
pub fn system(command: &str) -> i32 {
    use std::os::windows::ffi::OsStrExt;

    extern "C" {
        pub fn _wsystem(command: *const std::ffi::c_ushort) -> std::ffi::c_int;
    }

    if command.find("\0").is_some() {
        return -1;
    }

    let wide: Vec<u16> = std::ffi::OsStr::new(command)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    unsafe { _wsystem(wide.as_ptr()) }
}
