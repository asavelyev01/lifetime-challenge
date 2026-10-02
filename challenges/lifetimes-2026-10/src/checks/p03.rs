//! P03 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p03_fields::*;
/// let fields: Vec<Field<'static>> = {
///     let line = String::from("a=1 b");
///     parse_owned(&line).unwrap()
/// };
/// assert_eq!(fields[0].value(), Some("1"));
/// // 'static data can be moved to another thread:
/// let names = std::thread::spawn(move || fields.iter().map(|f| f.name().len()).sum::<usize>())
///     .join()
///     .unwrap();
/// assert_eq!(names, 2);
/// ```
pub fn owned_fields_are_static() {}

/// ```
/// use lifetimes_2026_10::p03_fields::*;
/// fn detach(line: &str) -> Field<'static> {
///     parse_fields(line).unwrap().remove(0).into_owned()
/// }
/// let f = detach(&String::from("k=v"));
/// assert_eq!(f.name(), "k");
/// ```
pub fn into_owned_detaches_from_the_line() {}

/// ```compile_fail
/// use lifetimes_2026_10::p03_fields::*;
/// let fields;
/// {
///     let line = String::from("a=1");
///     fields = parse_fields(&line).unwrap();
/// }
/// println!("{fields:?}");
/// ```
pub fn borrowed_fields_cannot_outlive_the_line() {}
