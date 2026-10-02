//! HARNESS — do not edit.
//!
//! The stub API can only be called once per interner/vector, so the runtime tests here are
//! deliberately written to compile against both the stub and a fixed API. The rest of the
//! behaviour is checked by the doctests in `src/checks/p06.rs`.
use lifetimes_2026_10::p06_interner::*;
use std::time::{Duration, Instant};

#[test]
fn intern_all_assigns_dense_first_seen_ids() {
    let text = String::from("  b a\tb c\n a  ");
    let mut interner = Interner::new();
    assert_eq!(intern_all(&mut interner, &text), [0, 1, 0, 2, 1]);
}

#[test]
fn intern_all_empty() {
    let mut interner = Interner::new();
    assert_eq!(intern_all(&mut interner, " \n "), Vec::<usize>::new());
}

#[test]
fn intern_all_is_case_and_unicode_exact() {
    let mut interner = Interner::new();
    assert_eq!(intern_all(&mut interner, "A a é e\u{301} a A"), [0, 1, 2, 3, 1, 0]);
}

#[test]
fn intern_returns_ids_in_order() {
    let mut interner = Interner::new();
    assert_eq!(interner.intern("first"), 0);
}

#[test]
fn lookup_on_empty_interner() {
    let interner = Interner::new();
    assert_eq!(interner.lookup("nope"), None);
    assert!(interner.is_empty());
}

#[test]
fn resolve_on_empty_interner() {
    let interner = Interner::new();
    assert_eq!(interner.resolve(0), None);
}

#[test]
fn extend_words_counts() {
    let text = String::from(" one two\tthree\n");
    let mut words = vec!["zero"];
    assert_eq!(extend_words(&mut words, &text), 3);
}

#[test]
fn extend_words_empty_text() {
    let mut words = vec!["zero"];
    assert_eq!(extend_words(&mut words, "  "), 0);
}

#[test]
fn intern_all_is_fast_for_many_duplicates() {
    let mut text = String::new();
    for i in 0..300_000u32 {
        text.push_str(&format!("w{} ", i.wrapping_mul(2_654_435_761) % 30_000));
    }
    let mut interner = Interner::new();
    let start = Instant::now();
    let ids = intern_all(&mut interner, &text);
    let took = start.elapsed();
    assert_eq!(ids.len(), 300_000);
    assert_eq!(ids.iter().max(), Some(&29_999), "30 000 distinct words");
    assert!(took < Duration::from_secs(3), "interning took {took:?}; must be O(1) per word");
}
