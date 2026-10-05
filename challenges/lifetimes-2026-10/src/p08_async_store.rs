//! # P08 — Futures that borrow                (difficulty: ★★★★☆)
//!
//! Async code adds a twist to lifetimes: a future holds on to everything it captures until it
//! completes, an `async fn`'s future captures **all** of its arguments, `tokio::spawn`
//! requires `'static` futures, and a callback that returns a future can't (with plain `Fn`
//! traits) let that future borrow the callback's arguments.
//!
//! [`Store`] is provided (don't modify it): an in-memory key/value store whose `get` is async
//! and records how many lookups are in flight concurrently.
//!
//! ## Tasks
//!
//! * `lookup(store, key)` — `store.get(key)`. The returned future must borrow only the store,
//!   **not** `key`: it must be fine to drop the key before awaiting
//!   (`lookup(&store, &String::from("k")).await` written across two statements).
//!   You may change the signature (it doesn't have to stay an `async fn`), but it must remain
//!   callable as `lookup(&store, key).await` and return `Option<&'s str>`.
//! * `lookup_all(store, keys)` — looks up all keys **concurrently** (all lookups in flight at
//!   the same time, on the current task — no spawning), results in key order.
//! * `spawn_lookups(store, keys)` — spawns one tokio task per key, each returning an owned
//!   copy of the value. Must spawn immediately (the tasks run even if nobody awaits the
//!   handles). Handles are returned in key order.
//! * `for_each_found(store, keys, f)` — for each key in order: look it up; if found, call
//!   `f(key, value)` and await the returned future before moving on. Returns how many keys
//!   were found. The current bound doesn't let `f`'s future borrow `key` or the closure's own
//!   captures (see `src/checks/p08.rs`) — fix the bound. It must stay a generic parameter `F`
//!   and keep accepting plain closures that return a future.

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::task::JoinHandle;

/// Provided — don't modify.
#[derive(Debug, Default)]
pub struct Store {
    data: HashMap<String, String>,
    in_flight: AtomicUsize,
    max_in_flight: AtomicUsize,
    calls: AtomicUsize,
}

impl Store {
    pub fn new<K: Into<String>, V: Into<String>>(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
        Store {
            data: pairs
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }

    pub async fn get(&self, key: &str) -> Option<&str> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let now = self.in_flight.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_in_flight.fetch_max(now, Ordering::SeqCst);
        tokio::task::yield_now().await;
        tokio::task::yield_now().await;
        self.in_flight.fetch_sub(1, Ordering::SeqCst);
        self.data.get(key).map(String::as_str)
    }

    /// Total number of `get` calls so far.
    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }

    /// The highest number of `get` futures that were in flight at the same time.
    pub fn max_in_flight(&self) -> usize {
        self.max_in_flight.load(Ordering::SeqCst)
    }
}

/// Looks `key` up in `store` (via `Store::get`).
///
/// To fix: the returned future must borrow only `store`, **not** `key` — it must be fine to drop
/// the key before awaiting. The future must be `Send`. Must stay callable as
/// `lookup(&store, key).await` and resolve to `Option<&'s str>` (it doesn't have to stay an
/// `async fn`).
// TODO(challenge): implement + fix the signature
pub async fn lookup<'s>(store: &'s Store, key: &str) -> Option<&'s str> {
    let _ = (store, key);
    todo!("p08: lookup")
}

/// Looks up all `keys` **concurrently** — all `Store::get` calls in flight at the same time, on
/// the current task (no spawning) — and returns the results in key order. Exactly one `get` per
/// key. The results borrow from `store` only (not from `keys`).
// TODO(challenge): implement
pub async fn lookup_all<'s>(store: &'s Store, keys: &[&str]) -> Vec<Option<&'s str>> {
    let _ = (store, keys);
    todo!("p08: lookup_all")
}

/// Spawns one tokio task per key (immediately — the tasks must run even if the handles are
/// dropped), each resolving to an **owned** copy of the value. Returns the handles in key order.
///
/// Spawned tasks must be `'static`: they can't borrow `keys` or a `&Store`.
// TODO(challenge): implement
pub fn spawn_lookups(store: Arc<Store>, keys: &[&str]) -> Vec<JoinHandle<Option<String>>> {
    let _ = (store, keys);
    todo!("p08: spawn_lookups")
}

/// For each key in order: look it up; if found, call `f(key, value)` and **await** the returned
/// future before looking up the next key. Returns how many keys were found.
///
/// To fix the bound: the future returned by `f` must be allowed to borrow `key` and the
/// closure's own captures (e.g. an `async |k, v| { …; log.push(..) }` closure), while `key`
/// still must not escape the call. Keep `F` a generic parameter and keep accepting plain
/// closures that return a future.
// TODO(challenge): implement + fix the bound
pub async fn for_each_found<'s, F, Fut>(store: &'s Store, keys: &[&str], f: F) -> usize
where
    F: FnMut(&str, &'s str) -> Fut,
    Fut: Future<Output = ()>,
{
    let _ = (store, keys, f);
    todo!("p08: for_each_found")
}
