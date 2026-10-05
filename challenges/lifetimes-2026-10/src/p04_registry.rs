//! # P04 — Mutable borrows that the borrow checker fights      (difficulty: ★★☆☆☆)
//!
//! A small score registry backed by a `Vec` (insertion order matters, so no `HashMap`).
//! Every method here has a natural first attempt that the borrow checker rejects —
//! find formulations that compile **without `unsafe`** (forbidden crate-wide), without
//! cloning entries, and without looking a name up more than twice per call.
//!
//! ## API
//!
//! * `get(name)` / `get_mut(name)` — the entry with that name, if any.
//! * `get_or_insert(name)` — the existing entry, or a new `Entry { name, score: 0, tags: [] }`
//!   appended at the end.
//! * `pair_mut(a, b)` — mutable references to two **different** entries, in argument order.
//!   `None` if either is missing or `a == b`.
//! * `transfer(from, to, amount)` — moves `amount` points from `from` to `to`. Errors, checked
//!   in this order: `amount <= 0` → `InvalidAmount`; `from == to` → `SameEntry`;
//!   `from` missing → `Unknown(from)`; `to` missing → `Unknown(to)`;
//!   `from.score < amount` → `Insufficient { available: from.score }`.
//!   On error nothing changes.
//! * `merge(from, into)` — removes `from` from the registry, adds its score to `into`, and
//!   appends those of its tags that `into` doesn't have yet (keeping `from`'s tag order).
//!   Returns the updated `into` entry. Errors in order: `SameEntry`, `Unknown(from)`,
//!   `Unknown(into)`. The relative order of the remaining entries is preserved.
//! * `top_by_tag(tag)` — the entry with the highest score among those having `tag`; ties go
//!   to the earliest inserted. `None` if no entry has the tag.
//!
//! ## Example
//!
//! ```text
//! get_or_insert("ann").score += 5;   // [ann:5]
//! get_or_insert("bob");               // [ann:5, bob:0]
//! transfer("ann", "bob", 2)?;         // [ann:3, bob:2]
//! merge("ann", "bob")?.score          // == 5, registry is now [bob:5]
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub score: i64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    InvalidAmount,
    SameEntry,
    Unknown(String),
    Insufficient { available: i64 },
}

#[derive(Debug, Default)]
pub struct Registry {
    entries: Vec<Entry>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// All entries in insertion order.
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// The entry named `name`, or `None`.
    // TODO(challenge): implement
    pub fn get(&self, name: &str) -> Option<&Entry> {
        let _ = name;
        todo!("p04: get")
    }

    /// Mutable access to the entry named `name`, or `None`.
    // TODO(challenge): implement
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Entry> {
        let _ = name;
        todo!("p04: get_mut")
    }

    /// The entry named `name`; if there is none, first appends
    /// `Entry { name, score: 0, tags: vec![] }` at the end and returns that.
    ///
    /// Must not create duplicates. The obvious "`if let Some(e) = self.get_mut(name) { return e }`
    /// then push" version is rejected by the borrow checker — find one that compiles (no `unsafe`).
    // TODO(challenge): implement
    pub fn get_or_insert(&mut self, name: &str) -> &mut Entry {
        let _ = name;
        todo!("p04: get_or_insert")
    }

    /// Mutable references to two **different** entries at once, in argument order:
    /// `pair_mut("c", "a")` returns `(entry c, entry a)`.
    ///
    /// `None` if either name is missing or `a == b`. No `unsafe`, no cloning.
    // TODO(challenge): implement
    pub fn pair_mut(&mut self, a: &str, b: &str) -> Option<(&mut Entry, &mut Entry)> {
        let _ = (a, b);
        todo!("p04: pair_mut")
    }

    /// Moves `amount` points from `from`'s score to `to`'s score.
    ///
    /// Errors, checked in this order (the first that applies is returned):
    /// 1. `amount <= 0` → `InvalidAmount`
    /// 2. `from == to` → `SameEntry`
    /// 3. `from` missing → `Unknown(from)`
    /// 4. `to` missing → `Unknown(to)`
    /// 5. `from.score < amount` → `Insufficient { available: from.score }`
    ///
    /// On error nothing changes. Transferring exactly the whole balance is allowed.
    // TODO(challenge): implement
    pub fn transfer(&mut self, from: &str, to: &str, amount: i64) -> Result<(), RegistryError> {
        let _ = (from, to, amount);
        todo!("p04: transfer")
    }

    /// Merges `from` into `into`: removes the `from` entry from the registry, adds its score to
    /// `into`, and appends those of `from`'s tags that `into` doesn't have yet (in `from`'s order).
    /// Returns the updated `into` entry.
    ///
    /// Errors, in order: `from == into` → `SameEntry`; `from` missing → `Unknown(from)`;
    /// `into` missing → `Unknown(into)`. On error nothing changes.
    ///
    /// The remaining entries keep their relative order (watch out: removing an entry shifts the
    /// indices of the ones after it).
    // TODO(challenge): implement
    pub fn merge(&mut self, from: &str, into: &str) -> Result<&mut Entry, RegistryError> {
        let _ = (from, into);
        todo!("p04: merge")
    }

    /// Among the entries having `tag`, the one with the highest score; ties go to the
    /// **earliest inserted**. `None` if no entry has the tag. Negative scores count normally.
    // TODO(challenge): implement
    pub fn top_by_tag(&self, tag: &str) -> Option<&Entry> {
        let _ = tag;
        todo!("p04: top_by_tag")
    }
}
