//! Rust lifetimes challenge — see `CHALLENGE.md`.
//!
//! HARNESS FILE — do not edit. Problems live in `src/pNN_*.rs`.
#![forbid(unsafe_code)]

pub mod p01_signatures;
pub mod p02_lexer;
pub mod p03_fields;
pub mod p04_registry;
pub mod p05_scanner;
pub mod p06_interner;
pub mod p07_dispatcher;
pub mod p08_async_store;
pub mod p09_lending;
pub mod p10_pipeline;

/// Compile-time checks, run as doctests (`cargo test --doc`). Harness — do not edit.
#[doc(hidden)]
pub mod checks;
