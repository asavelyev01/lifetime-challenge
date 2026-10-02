//! HARNESS — do not edit.
//!
//! Every problem has a `pNN.rs` module here whose doc comments are doctests:
//!
//! * plain blocks must **compile and run** — they encode how the API has to be usable;
//! * `compile_fail` blocks must be **rejected by the compiler** — they keep the API sound
//!   (no `'static` cheats, no returning data that outlives its owner, …).
//!
//! Note: a `compile_fail` check may already pass on the unsolved stub. That's expected;
//! it exists to stop "fixes" that loosen the API too much.

pub mod p01;
pub mod p02;
pub mod p03;
pub mod p04;
pub mod p05;
pub mod p06;
pub mod p07;
pub mod p08;
pub mod p09;
pub mod p10;

/// Runs a future to completion on a fresh current-thread tokio runtime.
pub fn block_on<F: std::future::Future>(fut: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
        .block_on(fut)
}
