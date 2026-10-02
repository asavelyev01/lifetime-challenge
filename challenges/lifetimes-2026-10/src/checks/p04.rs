//! P04 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p04_registry::*;
/// let mut reg = Registry::new();
/// reg.get_or_insert("a").score = 3;
/// reg.get_or_insert("b").score = 7;
/// let (a, b) = reg.pair_mut("a", "b").unwrap();
/// std::mem::swap(&mut a.score, &mut b.score); // two live &mut at once
/// assert_eq!(reg.get("a").unwrap().score, 7);
/// let top = reg.top_by_tag("none");
/// assert!(top.is_none());
/// ```
pub fn two_mutable_entries_at_once() {}

/// ```compile_fail
/// use lifetimes_2026_10::p04_registry::*;
/// let mut reg = Registry::new();
/// let a = reg.get_or_insert("a");
/// let b = reg.get_or_insert("b"); // `a` is still alive below
/// a.score += b.score;
/// ```
pub fn get_or_insert_keeps_the_registry_borrowed() {}
