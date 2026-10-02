//! P01 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p01_signatures::strip;
/// let version = String::from("v1.2.3");
/// let bare = strip(&version, &String::from("v")); // the prefix is a temporary
/// assert_eq!(bare, "1.2.3");
/// ```
pub fn strip_prefix_may_be_a_temporary() {}

/// ```compile_fail
/// use lifetimes_2026_10::p01_signatures::strip;
/// let out;
/// {
///     let version = String::from("v1");
///     out = strip(&version, "v");
/// }
/// println!("{out}");
/// ```
pub fn strip_result_cannot_outlive_its_input() {}

/// ```
/// use lifetimes_2026_10::p01_signatures::longest;
/// let text = String::from("a quick brown fox");
/// let best = {
///     let words: Vec<&str> = text.split(' ').collect();
///     longest(&words) // the Vec dies here, the words live on in `text`
/// };
/// assert_eq!(best, Some("quick"));
/// ```
pub fn longest_may_outlive_the_slice() {}

/// ```compile_fail
/// use lifetimes_2026_10::p01_signatures::longest;
/// let best;
/// {
///     let text = String::from("a bb");
///     let words: Vec<&str> = text.split(' ').collect();
///     best = longest(&words);
/// }
/// println!("{best:?}");
/// ```
pub fn longest_cannot_outlive_the_strings() {}

/// ```
/// use lifetimes_2026_10::p01_signatures::lookup;
/// let cfg = String::from("host = example.org\nport=8080");
/// let port = lookup(&cfg, &format!("po{}", "rt")); // the key is a temporary
/// assert_eq!(port, Some("8080"));
/// ```
pub fn lookup_key_may_be_a_temporary() {}

/// ```compile_fail
/// use lifetimes_2026_10::p01_signatures::lookup;
/// let v;
/// {
///     let cfg = String::from("a=1");
///     v = lookup(&cfg, "a");
/// }
/// println!("{v:?}");
/// ```
pub fn lookup_result_cannot_outlive_config() {}

/// ```
/// use lifetimes_2026_10::p01_signatures::plural_suffix;
/// let suffix: &'static str = {
///     let word = String::from("box");
///     plural_suffix(&word, 2)
/// };
/// assert_eq!(suffix, "es");
/// ```
pub fn plural_suffix_is_static() {}
