//! # P01 — Relax the signatures            (difficulty: ★☆☆☆☆, warm-up)
//!
//! Every function below compiles, but its signature ties lifetimes together that have
//! nothing to do with each other. Callers can't use the results the way they'd
//! naturally want to (see `src/checks/p01.rs`).
//!
//! Your job, for every function:
//! 1. implement the body;
//! 2. fix the signature so that it is **exactly as permissive as the body allows** —
//!    no more (returning owned `String`s, leaking or `'static` cheats are wrong), no less.
//!
//! Keep the function names, parameter order and return *shapes* (`&str`, `Option<&str>`)
//! as they are; only lifetimes may change.
//!
//! ## Specs
//!
//! * `strip(s, prefix)` — `s` without `prefix` if `s` starts with it, otherwise `s`
//!   unchanged. An empty prefix returns `s`.
//!   `strip("v1.2", "v") == "1.2"`, `strip("1.2", "v") == "1.2"`.
//!
//! * `longest(items)` — the longest string, measured in **chars** (not bytes); on ties the
//!   first one wins; `None` for an empty slice.
//!   `longest(&["ab", "ééé", "xyz"]) == Some("ééé")`.
//!
//! * `lookup(config, key)` — `config` is a list of lines separated by `'\n'`. Each line is
//!   trimmed (`str::trim`); empty lines and lines starting with `#` are skipped; the
//!   remaining lines are split at the **first** `'='` (lines without `'='` are ignored).
//!   Returns the trimmed value of the first line whose trimmed key equals `key`
//!   (case-sensitive). `lookup("a = 1\nb=2=3", "b") == Some("2=3")`.
//!
//! * `plural_suffix(word, count)` — `""` if `count == 1`; otherwise `"es"` if `word` ends
//!   with `s`, `x`, `z`, `ch` or `sh` (ASCII case-insensitive), else `"s"`.
//!   `plural_suffix("Box", 2) == "es"`, `plural_suffix("cat", 0) == "s"`.

// TODO(challenge): implement + fix the signature
pub fn strip<'a>(s: &'a str, prefix: &'a str) -> &'a str {
    let _ = (s, prefix);
    todo!("p01: strip")
}

// TODO(challenge): implement + fix the signature
pub fn longest<'a>(items: &'a [&'a str]) -> Option<&'a str> {
    let _ = items;
    todo!("p01: longest")
}

// TODO(challenge): implement + fix the signature
pub fn lookup<'a>(config: &'a str, key: &'a str) -> Option<&'a str> {
    let _ = (config, key);
    todo!("p01: lookup")
}

// TODO(challenge): implement + fix the signature
pub fn plural_suffix<'a>(word: &'a str, count: usize) -> &'a str {
    let _ = (word, count);
    todo!("p01: plural_suffix")
}
