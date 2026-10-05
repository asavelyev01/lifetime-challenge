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

/// Returns `s` without `prefix` if `s` starts with it, otherwise `s` unchanged.
/// An empty prefix returns `s`. The result is always a sub-slice of `s` (no copying).
///
/// Examples: `strip("v1.2", "v") == "1.2"`, `strip("1.2", "v") == "1.2"`, `strip("abc", "abc") == ""`.
///
/// Lifetimes: the result may outlive `prefix` (e.g. a temporary `&String::from("v")`),
/// but must not outlive `s`.
// TODO(challenge): implement + fix the signature
pub fn strip<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.strip_prefix(prefix).unwrap_or(s)
}

/// Returns the longest string of `items`, measured in **chars** (`chars().count()`, not bytes).
/// On ties the **first** one wins. `None` for an empty slice.
///
/// Examples: `longest(&["ab", "ééé", "xyz"]) == Some("ééé")`, `longest(&["aa", "bb"]) == Some("aa")`.
///
/// Lifetimes: the result may outlive the slice itself (e.g. a temporary `Vec<&str>`),
/// but must not outlive the strings in it.
// TODO(challenge): implement + fix the signature
pub fn longest<'a, 'b>(items: &'a [&'b str]) -> Option<&'b str> {
    fn length(s: &str) -> usize {
        s.chars().count()
    }

    let mut long: Option<&'b str> = None;

    for s in items {
        if long.is_none_or(|l| length(s) > length(l)) {
            long = Some(*s)
        }
    }

    long
}

/// Looks up `key` in a `key = value` config text.
///
/// `config` is split into lines on `'\n'`. For each line, in order:
/// 1. trim it (`str::trim`);
/// 2. skip it if it is empty or (after trimming) starts with `#`;
/// 3. split at the **first** `'='` — lines without `'='` are ignored;
/// 4. if the trimmed key part equals `key` exactly (case-sensitive, `key` is not trimmed),
///    return the trimmed value.
///
/// Returns `None` if no line matches. Examples:
/// `lookup("a = 1\nb=2=3", "b") == Some("2=3")`, `lookup("  # k = 1", "# k") == None`,
/// `lookup("k=", "k") == Some("")`.
///
/// Lifetimes: the result may outlive `key`, but must not outlive `config`.
// TODO(challenge): implement + fix the signature
pub fn lookup<'a>(config: &'a str, key: &str) -> Option<&'a str> {
    for line in config.split('\n') {
        if !line.starts_with('#')
            && let Some((k, v)) = line.split_once('=')
            && k.trim() == key.trim()
        {
            return Some(v.trim());
        }
    }

    None
}

/// Returns the English plural suffix for `word` used with `count`:
/// * `""` if `count == 1`;
/// * otherwise `"es"` if `word` ends with `s`, `x`, `z`, `ch` or `sh` (ASCII case-insensitive);
/// * otherwise `"s"`.
///
/// Examples: `plural_suffix("Box", 2) == "es"`, `plural_suffix("cat", 0) == "s"`,
/// `plural_suffix("bus", 1) == ""`.
///
/// Lifetimes: the result never borrows from `word` — it must be usable after `word` is dropped.
// TODO(challenge): implement + fix the signature
pub fn plural_suffix(word: &str, count: usize) -> &'static str {
    if count == 1 {
        ""
    } else if ["s", "x", "z", "ch", "sh"].iter().any({
        let word = word.to_ascii_lowercase();
        move |s| word.ends_with(s)
    }) {
        "es"
    } else {
        "s"
    }
}
