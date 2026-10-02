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
            data: pairs.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
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

// An `async fn`'s future captures *all* arguments, so `key` would have to outlive the await.
// Instead: a plain fn that copies the key up front and returns a future that only captures
// `'s` (`use<'s>` — in edition 2024, `+ 's` alone would still capture the key's lifetime).
pub fn lookup<'s>(store: &'s Store, key: &str) -> impl Future<Output = Option<&'s str>> + Send + use<'s> {
    let key = key.to_owned();
    async move { store.get(&key).await }
}

// `join_all` polls all lookups on the current task; the futures may borrow freely because
// they don't need to be `'static` (unlike spawned tasks).
pub async fn lookup_all<'s>(store: &'s Store, keys: &[&str]) -> Vec<Option<&'s str>> {
    futures::future::join_all(keys.iter().map(|k| store.get(k))).await
}

// `tokio::spawn` needs `'static` futures: each task gets its own `Arc` and an owned key, and
// returns an owned value (it can't hand out a borrow of a store it doesn't outlive).
pub fn spawn_lookups(store: Arc<Store>, keys: &[&str]) -> Vec<JoinHandle<Option<String>>> {
    keys.iter()
        .map(|key| {
            let (store, key) = (Arc::clone(&store), key.to_string());
            tokio::spawn(async move { store.get(&key).await.map(str::to_owned) })
        })
        .collect()
}

// With `F: FnMut(&str, &'s str) -> Fut`, the single type `Fut` can't depend on the
// (higher-ranked) lifetime of `key`, nor borrow from the closure itself. `AsyncFnMut` can:
// its future type is generic over the borrow of both the arguments and the closure.
pub async fn for_each_found<'s, F>(store: &'s Store, keys: &[&str], mut f: F) -> usize
where
    F: AsyncFnMut(&str, &'s str),
{
    let mut found = 0;
    for &key in keys {
        if let Some(value) = store.get(key).await {
            f(key, value).await;
            found += 1;
        }
    }
    found
}
