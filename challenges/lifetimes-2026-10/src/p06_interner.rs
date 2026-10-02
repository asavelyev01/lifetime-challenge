//! # P06 — Borrowed forever                  (difficulty: ★★★☆☆)
//!
//! A string interner that stores *borrowed* strings: it maps each distinct `&'a str` to a
//! dense id (`0, 1, 2, …` in first-seen order) and back.
//!
//! The signatures below look plausible and compile, but they make the API unusable: after
//! the first `intern` the interner is mutably borrowed **for the rest of its life**, and
//! `extend_words` can only be called once per vector. Read the compile checks in
//! `src/checks/p06.rs` (both kinds!), then fix every signature marked `TODO(challenge)` and
//! implement the bodies.
//!
//! Constraints:
//! * `Interner<'a>` must stay **covariant** in `'a` (an `Interner<'long>` must be usable
//!   where an `Interner<'short>` is expected — there is a check for that).
//! * Don't copy the strings: `resolve` returns the very slice that was interned.
//! * `intern` and `lookup` must be O(1) expected: the test interns 300 000 words
//!   (30 000 distinct) and must finish in well under a few seconds even in debug mode.
//!
//! ## API
//!
//! * `intern(s)` — the id of `s`, assigning the next free id if it's new.
//! * `lookup(s)` — the id of `s` if it was interned, without inserting.
//! * `resolve(id)` — the interned string for `id`, if any.
//! * `intern_all(interner, text)` — interns every `split_whitespace` word of `text`, returns
//!   their ids in order.
//! * `extend_words(dst, text)` — pushes every `split_whitespace` word of `text` onto `dst`,
//!   returns how many were pushed.
//!
//! Example: interning `"b"`, `"a"`, `"b"` returns ids `0, 1, 0`; `resolve(1) == Some("a")`.

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Interner<'a> {
    ids: HashMap<&'a str, usize>,
    strings: Vec<&'a str>,
}

impl<'a> Interner<'a> {
    pub fn new() -> Self {
        Interner { ids: HashMap::new(), strings: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    // `&mut self` borrows the interner only for this call; `s: &'a str` is what gets stored.
    // With `&'a mut self` the interner would stay mutably borrowed for its whole lifetime `'a`.
    pub fn intern(&mut self, s: &'a str) -> usize {
        *self.ids.entry(s).or_insert_with(|| {
            self.strings.push(s);
            self.strings.len() - 1
        })
    }

    // The query string is only compared, so it may live as briefly as the call.
    pub fn lookup(&self, s: &str) -> Option<usize> {
        self.ids.get(s).copied()
    }

    // The result borrows the original text (`'a`), not the interner.
    pub fn resolve(&self, id: usize) -> Option<&'a str> {
        self.strings.get(id).copied()
    }
}

// `&'a mut Interner<'a>` would borrow the interner for as long as it *stores* data — i.e.
// forever. `&mut T<'a>` is invariant in `'a`, so the compiler can't shrink it to help either.
pub fn intern_all<'a>(interner: &mut Interner<'a>, text: &'a str) -> Vec<usize> {
    text.split_whitespace().map(|w| interner.intern(w)).collect()
}

// Same story: the borrow of the Vec and the lifetime of its elements are independent.
pub fn extend_words<'a>(dst: &mut Vec<&'a str>, text: &'a str) -> usize {
    let before = dst.len();
    dst.extend(text.split_whitespace());
    dst.len() - before
}
