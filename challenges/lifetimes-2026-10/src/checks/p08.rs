//! P08 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::{checks::block_on, p08_async_store::*};
/// let store = Store::new([("a", "1")]);
/// let value = block_on(async {
///     let fut = lookup(&store, &String::from("a")); // the key is a temporary
///     fut.await
/// });
/// assert_eq!(value, Some("1"));
/// ```
pub fn lookup_key_may_be_dropped_before_awaiting() {}

/// ```
/// use lifetimes_2026_10::{checks::block_on, p08_async_store::*};
/// fn assert_send<T: Send>(t: T) -> T { t }
/// let store = Store::new([("a", "1")]);
/// let key = String::from("a");
/// let fut = assert_send(lookup(&store, &key));
/// drop(key);
/// assert_eq!(block_on(fut), Some("1"));
/// ```
pub fn lookup_future_is_send_and_independent_of_key() {}

/// ```
/// use lifetimes_2026_10::{checks::block_on, p08_async_store::*};
/// let store = Store::new([("a", "1"), ("b", "2")]);
/// let mut log: Vec<String> = Vec::new();
/// let found = block_on(for_each_found(&store, &["a", "zz", "b"], async |key, value| {
///     tokio::task::yield_now().await;
///     log.push(format!("{key}={value}")); // borrows `key` and `log` across an await
/// }));
/// assert_eq!(found, 2);
/// assert_eq!(log, ["a=1", "b=2"]);
/// ```
pub fn for_each_found_callback_future_may_borrow() {}

/// ```
/// use lifetimes_2026_10::{checks::block_on, p08_async_store::*};
/// let store = Store::new([("k", "v")]);
/// let values = {
///     let keys = vec![String::from("k"), String::from("x")];
///     let refs: Vec<&str> = keys.iter().map(String::as_str).collect();
///     block_on(lookup_all(&store, &refs)) // keys die here, values borrow the store
/// };
/// assert_eq!(values, [Some("v"), None]);
/// ```
pub fn lookup_all_results_outlive_the_keys() {}

/// ```compile_fail
/// use lifetimes_2026_10::{checks::block_on, p08_async_store::*};
/// let value;
/// {
///     let store = Store::new([("a", "1")]);
///     value = block_on(lookup(&store, "a"));
/// }
/// println!("{value:?}");
/// ```
pub fn lookup_result_cannot_outlive_the_store() {}

/// ```compile_fail
/// use lifetimes_2026_10::{checks::block_on, p08_async_store::*};
/// let store = Store::new([("a", "1")]);
/// let mut kept: Vec<&str> = Vec::new();
/// block_on(for_each_found(&store, &["a"], async |key, _value| {
///     kept.push(key); // the key is only lent for the duration of the call
/// }));
/// println!("{kept:?}");
/// ```
pub fn for_each_found_callback_cannot_keep_the_key() {}
