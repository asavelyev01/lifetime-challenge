//! # P09 — Lending iterators                  (difficulty: ★★★★★)
//!
//! A *lending* iterator hands out items that borrow from the iterator itself, so each item
//! must be dropped before the next `next()` call. That's impossible to express with
//! `Iterator` (whose `Item` can't mention the `&mut self` borrow) but possible with a generic
//! associated type, as below.
//!
//! ## Part A — implement
//!
//! * `windows_mut(slice, size)` — overlapping **mutable** windows of `size` consecutive
//!   elements, step 1: a slice of length `n` yields `n - size + 1` windows (none if
//!   `size > n`). `size == 0` panics (already implemented).
//! * `lines(reader)` — lends each line of a `BufRead` as `&str`, reusing **one internal
//!   `String` buffer** (no allocation per line). Lines are split on `'\n'`; one trailing `'\r'`
//!   is removed; a final line without `'\n'` is still yielded; `""` input yields nothing.
//!   `"a\r\n\nb"` → `"a"`, `""`, `"b"`. An I/O error (including invalid UTF-8) is yielded
//!   once as `Err`, after which the iterator returns `None`.
//! * `enumerate(it)` — pairs each item with its index `0, 1, 2, …`.
//! * `for_each(it, f)` — calls `f` on every item.
//! * `count_where(it, pred)` — counts the items for which `pred(&item)` is true.
//!
//! ## Part B — make it actually usable
//!
//! With the definitions below, `for_each`/`count_where` compile, and they even work for
//! iterators that own their data — but calling them with a `windows_mut` over a local
//! `Vec` fails with *"argument requires that `v` is borrowed for `'static`"*
//! (see `src/checks/p09.rs`). Find out why, and fix it.
//!
//! You may restructure **anything** in this file, including the trait itself, as long as:
//! * the trait is still called `LendingIterator` and has a method `next(&mut self)`, so that
//!   `while let Some(item) = it.next() { … }` works with `LendingIterator` in scope;
//! * `windows_mut`, `lines`, `enumerate`, `for_each`, `count_where` keep their names and are
//!   still called like `for_each(it, |item| …)` — closure parameter types must be inferred;
//! * items still borrow from the iterator (a lent window can't be kept across `next()`).
//!
//! No `unsafe` (forbidden crate-wide), no `'static` bounds that the checks would reject.

use std::io::{self, BufRead};

/// An iterator whose items may borrow from the iterator itself (each item must be dropped
/// before calling `next()` again).
///
/// Part B: with this definition, `for_each`/`count_where` demand `I: 'static`, so they reject
/// iterators over local data (`windows_mut(&mut v, 2)`). Fix that — you may restructure this
/// trait. Keep its name and `next(&mut self)`.
// TODO(challenge): Part B — this trait has a problem
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    /// The next item, or `None` when the iterator is exhausted.
    fn next(&mut self) -> Option<Self::Item<'_>>;
}

pub struct WindowsMut<'s, T> {
    slice: &'s mut [T],
    size: usize,
    pos: usize,
}

pub fn windows_mut<T>(slice: &mut [T], size: usize) -> WindowsMut<'_, T> {
    assert!(size > 0, "window size must be positive");
    WindowsMut {
        slice,
        size,
        pos: 0,
    }
}

impl<'s, T> LendingIterator for WindowsMut<'s, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;

    /// The next window of `size` consecutive elements (step 1), as a mutable slice. A slice of
    /// length `n` yields `n - size + 1` windows (none if `size > n`); `None` forever afterwards.
    /// Writes through a window are visible in the following windows.
    // TODO(challenge): implement
    fn next(&mut self) -> Option<&mut [T]> {
        let _ = (&self.slice, self.size, self.pos);
        todo!("p09: WindowsMut::next")
    }
}

pub struct Lines<R> {
    reader: R,
    buf: String,
}

pub fn lines<R: BufRead>(reader: R) -> Lines<R> {
    Lines {
        reader,
        buf: String::new(),
    }
}

impl<R: BufRead> LendingIterator for Lines<R> {
    type Item<'a>
        = io::Result<&'a str>
    where
        Self: 'a;

    /// The next line, lent from one internal `String` buffer reused for every line.
    ///
    /// Lines are split on `'\n'`; one trailing `'\r'` is removed; a final line without `'\n'` is
    /// still yielded; empty input yields nothing (`"x\n"` yields only `"x"`).
    /// An I/O error (including invalid UTF-8) is yielded once as `Err`; after it, and after the end
    /// of input, returns `None`.
    // TODO(challenge): implement
    fn next(&mut self) -> Option<io::Result<&str>> {
        let _ = (&self.reader, &self.buf);
        todo!("p09: Lines::next")
    }
}

pub struct Enumerate<I> {
    inner: I,
    count: usize,
}

pub fn enumerate<I: LendingIterator>(inner: I) -> Enumerate<I> {
    Enumerate { inner, count: 0 }
}

impl<I: LendingIterator> LendingIterator for Enumerate<I> {
    type Item<'a>
        = (usize, I::Item<'a>)
    where
        Self: 'a;

    /// The inner iterator's next item paired with its index (`0, 1, 2, …`).
    // TODO(challenge): implement
    fn next(&mut self) -> Option<Self::Item<'_>> {
        let _ = (&self.inner, self.count);
        todo!("p09: Enumerate::next")
    }
}

/// Calls `f` on every item of `it`, in order.
///
/// Part B: must work for iterators that borrow local data (`for_each(windows_mut(&mut v, 2), |w| …)`),
/// with the closure's parameter type inferred.
// TODO(challenge): implement (+ Part B)
pub fn for_each<I, F>(it: I, f: F)
where
    I: LendingIterator,
    F: FnMut(I::Item<'_>),
{
    let _ = (it, f);
    todo!("p09: for_each")
}

/// Counts the items of `it` for which `pred(&item)` returns `true`.
///
/// Part B: must work for iterators that borrow local data, with the closure's parameter type
/// inferred.
// TODO(challenge): implement (+ Part B)
pub fn count_where<I, P>(it: I, pred: P) -> usize
where
    I: LendingIterator,
    P: FnMut(&I::Item<'_>) -> bool,
{
    let _ = (it, pred);
    todo!("p09: count_where")
}
