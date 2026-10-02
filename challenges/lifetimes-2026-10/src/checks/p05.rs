//! P05 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p05_scanner::*;
/// let text = String::from("let x = if y { 1 } else { 2 };");
/// let spans: Vec<Span> = {
///     let keywords = vec!["if".to_string(), "else".to_string()];
///     Scanner::new(&keywords, &text).collect() // keywords die here
/// };
/// assert_eq!(spans.iter().map(|s| s.text).collect::<Vec<_>>(), ["if", "else"]);
/// ```
pub fn spans_outlive_the_keyword_list() {}

/// ```
/// use lifetimes_2026_10::p05_scanner::*;
/// let keywords = vec!["fn".to_string()];
/// let kw: &str = {
///     let text = String::from("fn f() {}");
///     let mut scanner = Scanner::new(&keywords, &text);
///     let span = scanner.next().unwrap();
///     scanner.keyword_of(&span) // text and scanner die here
/// };
/// assert_eq!(kw, "fn");
/// ```
pub fn keywords_outlive_the_text() {}

/// ```
/// use lifetimes_2026_10::p05_scanner::*;
/// let text = String::from("a b c");
/// let rest = {
///     let keywords = vec!["b".to_string()];
///     let mut scanner = Scanner::new(&keywords, &text);
///     scanner.next();
///     scanner.rest()
/// };
/// assert_eq!(rest, " c");
/// ```
pub fn rest_outlives_the_keyword_list() {}

/// ```compile_fail
/// use lifetimes_2026_10::p05_scanner::*;
/// let keywords = vec!["a".to_string()];
/// let spans: Vec<Span>;
/// {
///     let text = String::from("a b");
///     spans = Scanner::new(&keywords, &text).collect();
/// }
/// println!("{spans:?}");
/// ```
pub fn spans_cannot_outlive_the_text() {}

/// ```compile_fail
/// use lifetimes_2026_10::p05_scanner::*;
/// let text = String::from("a b");
/// let kw: &str;
/// {
///     let keywords = vec!["a".to_string()];
///     let mut scanner = Scanner::new(&keywords, &text);
///     let span = scanner.next().unwrap();
///     kw = scanner.keyword_of(&span);
/// }
/// println!("{kw}");
/// ```
pub fn keyword_cannot_outlive_the_keyword_list() {}
