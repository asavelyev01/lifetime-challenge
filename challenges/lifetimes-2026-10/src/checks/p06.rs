//! P06 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p06_interner::*;
/// let text = String::from("b a b");
/// let mut interner = Interner::new();
/// let ids: Vec<usize> = text.split(' ').map(|w| interner.intern(w)).collect();
/// assert_eq!(ids, [0, 1, 0]);
/// assert_eq!(interner.lookup(&String::from("a")), Some(1)); // temporary query string
/// let a = interner.resolve(1).unwrap();
/// drop(interner); // the resolved string borrows the text, not the interner
/// assert_eq!(a, "a");
/// ```
pub fn intern_many_times_then_resolve() {}

/// ```
/// use lifetimes_2026_10::p06_interner::*;
/// let text = String::from("x y x z");
/// let mut interner = Interner::new();
/// let ids = intern_all(&mut interner, &text);
/// let more = intern_all(&mut interner, "z w");
/// assert_eq!((ids, more), (vec![0, 1, 0, 2], vec![2, 3]));
/// assert_eq!(interner.len(), 4);
/// ```
pub fn intern_all_can_be_called_repeatedly() {}

/// ```
/// use lifetimes_2026_10::p06_interner::*;
/// let text = String::from("x y z");
/// let mut words = Vec::new();
/// extend_words(&mut words, &text);
/// extend_words(&mut words, "w");
/// assert_eq!(words, ["x", "y", "z", "w"]);
/// ```
pub fn extend_words_can_be_called_repeatedly() {}

/// ```
/// use lifetimes_2026_10::p06_interner::*;
/// let text = String::from(" one two\tthree\n");
/// let mut words = vec!["zero"];
/// assert_eq!(extend_words(&mut words, &text), 3);
/// assert_eq!(words, ["zero", "one", "two", "three"]);
/// assert_eq!(words[2].as_ptr(), text[5..].as_ptr(), "must borrow from the text, not copy");
/// ```
pub fn extend_words_borrows_without_copying() {}

/// ```
/// use lifetimes_2026_10::p06_interner::*;
/// fn shrink<'long: 'short, 'short>(i: Interner<'long>) -> Interner<'short> { i }
/// fn shrink_ref<'r, 'long: 'short, 'short>(i: &'r Interner<'long>) -> &'r Interner<'short> { i }
/// let i: Interner<'static> = Interner::new();
/// let _ = shrink_ref(&i);
/// let _ = shrink(i);
/// ```
pub fn interner_is_covariant() {}

/// ```compile_fail
/// use lifetimes_2026_10::p06_interner::*;
/// let mut interner = Interner::new();
/// let word;
/// {
///     let text = String::from("q");
///     interner.intern(&text);
///     word = interner.resolve(0).unwrap();
/// }
/// println!("{word}");
/// ```
pub fn resolved_cannot_outlive_the_text() {}

/// ```compile_fail
/// use lifetimes_2026_10::p06_interner::*;
/// let mut interner = Interner::new();
/// {
///     let text = String::from("q");
///     interner.intern(&text);
/// }
/// println!("{}", interner.len());
/// ```
pub fn interner_cannot_outlive_interned_text() {}

/// ```compile_fail
/// use lifetimes_2026_10::p06_interner::*;
/// let mut words = Vec::new();
/// {
///     let text = String::from("x y");
///     extend_words(&mut words, &text);
/// }
/// println!("{words:?}");
/// ```
pub fn words_cannot_outlive_their_text() {}
