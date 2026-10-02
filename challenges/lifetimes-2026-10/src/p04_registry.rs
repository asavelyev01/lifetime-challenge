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

    fn index_of(&self, name: &str) -> Option<usize> {
        self.entries.iter().position(|e| e.name == name)
    }

    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|e| e.name == name)
    }

    // The tempting `if let Some(e) = self.get_mut(name) { return e; } self.entries.push(..)`
    // is rejected by today's borrow checker (NLL "problem case #3": a borrow returned on one
    // path is considered live on the other). Looking up an *index* first sidesteps it.
    pub fn get_or_insert(&mut self, name: &str) -> &mut Entry {
        let i = match self.index_of(name) {
            Some(i) => i,
            None => {
                self.entries.push(Entry { name: name.to_string(), score: 0, tags: Vec::new() });
                self.entries.len() - 1
            }
        };
        &mut self.entries[i]
    }

    // `get_disjoint_mut` proves disjointness at runtime and hands out two `&mut` safely.
    pub fn pair_mut(&mut self, a: &str, b: &str) -> Option<(&mut Entry, &mut Entry)> {
        let (i, j) = (self.index_of(a)?, self.index_of(b)?);
        let [x, y] = self.entries.get_disjoint_mut([i, j]).ok()?;
        Some((x, y))
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: i64) -> Result<(), RegistryError> {
        if amount <= 0 {
            return Err(RegistryError::InvalidAmount);
        }
        if from == to {
            return Err(RegistryError::SameEntry);
        }
        let (i, j) = self.indices(from, to)?;
        let [src, dst] = self.entries.get_disjoint_mut([i, j]).expect("distinct names → distinct indices");
        if src.score < amount {
            return Err(RegistryError::Insufficient { available: src.score });
        }
        src.score -= amount;
        dst.score += amount;
        Ok(())
    }

    pub fn merge(&mut self, from: &str, into: &str) -> Result<&mut Entry, RegistryError> {
        if from == into {
            return Err(RegistryError::SameEntry);
        }
        let (i, j) = self.indices(from, into)?;
        // Removing `from` shifts every later entry one slot to the left.
        let gone = self.entries.remove(i);
        let target = &mut self.entries[if j > i { j - 1 } else { j }];
        target.score += gone.score;
        for tag in gone.tags {
            if !target.tags.contains(&tag) {
                target.tags.push(tag);
            }
        }
        Ok(target)
    }

    fn indices(&self, a: &str, b: &str) -> Result<(usize, usize), RegistryError> {
        let i = self.index_of(a).ok_or_else(|| RegistryError::Unknown(a.to_string()))?;
        let j = self.index_of(b).ok_or_else(|| RegistryError::Unknown(b.to_string()))?;
        Ok((i, j))
    }

    pub fn top_by_tag(&self, tag: &str) -> Option<&Entry> {
        // `max_by_key` keeps the *last* maximum; reversing makes ties go to the earliest.
        self.entries.iter().rev().filter(|e| e.tags.iter().any(|t| t == tag)).max_by_key(|e| e.score)
    }
}
