//! P09 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p09_lending::*;
/// let mut v = vec![1, 2, 3, 4];
/// for_each(windows_mut(&mut v, 2), |w| w[1] += w[0]); // prefix sums, in place
/// assert_eq!(v, [1, 3, 6, 10]);
/// ```
pub fn for_each_over_borrowed_windows() {}

/// ```
/// use lifetimes_2026_10::p09_lending::*;
/// let mut v = vec![3, 1, 4, 1, 5];
/// let n = count_where(windows_mut(&mut v, 2), |w| w[0] > w[1]);
/// assert_eq!(n, 2);
/// ```
pub fn count_where_over_borrowed_windows() {}

/// ```
/// use lifetimes_2026_10::p09_lending::*;
/// let mut v = vec![0; 4];
/// for_each(enumerate(windows_mut(&mut v, 3)), |(i, w)| w[2] = i + 10);
/// assert_eq!(v, [0, 0, 10, 11]);
/// ```
pub fn for_each_over_enumerated_borrowed_windows() {}

/// ```
/// use lifetimes_2026_10::p09_lending::*;
/// let text = String::from("keep\n# drop\nkeep too\n");
/// let n = count_where(lines(text.as_bytes()), |line| !line.as_ref().unwrap().starts_with('#'));
/// assert_eq!(n, 2);
/// let mut total = 0;
/// for_each(enumerate(lines(text.as_bytes())), |(i, line)| total += i * line.unwrap().len());
/// assert_eq!(total, 1 * 6 + 2 * 8);
/// ```
pub fn lines_over_a_borrowed_reader() {}

/// ```
/// use lifetimes_2026_10::p09_lending::*;
/// let mut v = vec![1i64, 2, 3];
/// let mut it = windows_mut(&mut v, 2);
/// let mut sums = vec![];
/// while let Some(w) = it.next() {
///     w[0] *= 10;
///     sums.push(w.iter().sum::<i64>());
/// }
/// assert_eq!(sums, [12, 23]);
/// assert_eq!(v, [10, 20, 3]);
/// ```
pub fn while_let_over_borrowed_windows() {}

/// ```compile_fail
/// use lifetimes_2026_10::p09_lending::*;
/// let mut v = vec![1, 2, 3];
/// let mut it = windows_mut(&mut v, 2);
/// let a = it.next().unwrap();
/// let b = it.next().unwrap(); // `a` is still alive: windows overlap!
/// a[1] = b[0];
/// ```
pub fn windows_cannot_be_held_across_next() {}

/// ```compile_fail
/// use lifetimes_2026_10::p09_lending::*;
/// let mut v = vec![1, 2, 3];
/// let mut kept = Vec::new();
/// for_each(windows_mut(&mut v, 2), |w| kept.push(w));
/// println!("{}", kept.len());
/// ```
pub fn for_each_cannot_keep_items() {}

/// ```compile_fail
/// use lifetimes_2026_10::p09_lending::*;
/// let mut it = lines("a\nb\n".as_bytes());
/// let first = it.next().unwrap().unwrap();
/// let _second = it.next();
/// println!("{first}"); // the buffer was reused
/// ```
pub fn lines_cannot_be_held_across_next() {}
