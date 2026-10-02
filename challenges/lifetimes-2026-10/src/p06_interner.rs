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

    // TODO(challenge): implement + fix the signature
    pub fn intern(&'a mut self, s: &'a str) -> usize {
        let _ = (s, &self.ids);
        todo!("p06: intern")
    }

    // TODO(challenge): implement + fix the signature
    pub fn lookup(&'a self, s: &'a str) -> Option<usize> {
        let _ = s;
        todo!("p06: lookup")
    }

    // TODO(challenge): implement + fix the signature
    pub fn resolve(&'a self, id: usize) -> Option<&'a str> {
        let _ = id;
        todo!("p06: resolve")
    }
}

// TODO(challenge): implement + fix the signature
pub fn intern_all<'a>(interner: &'a mut Interner<'a>, text: &'a str) -> Vec<usize> {
    let _ = (interner, text);
    todo!("p06: intern_all")
}

// TODO(challenge): implement + fix the signature
pub fn extend_words<'a>(dst: &'a mut Vec<&'a str>, text: &'a str) -> usize {
    let _ = (dst, text);
    todo!("p06: extend_words")
}
