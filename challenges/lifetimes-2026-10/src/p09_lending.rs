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

// Why the GAT version fails: `F: FnMut(I::Item<'_>)` means `for<'a> FnMut(I::Item<'a>)`, and
// `Item<'a>` is only well-formed when `I: 'a` (the GAT's `where Self: 'a`). A `for<'a>` binder
// can't carry that condition, so the compiler must prove it for *every* `'a` — including
// `'static` — i.e. it demands `I: 'static`.
//
// The fix: put the item type in a separate trait, parameterised by the lifetime, with a
// defaulted type parameter `&'a Self`. Mentioning `<I as Lending<'a>>` then implicitly
// mentions the type `&'a I`, whose well-formedness *implies* `I: 'a` — and implied bounds do
// work under `for<'a>` binders. So `for<'a> FnMut(Item<'a, I>)` only ranges over lifetimes
// for which `I: 'a`, and borrowed iterators are accepted.
// (See Sabrina Jewson, "The Better Alternative to Lifetime GATs".)
pub trait Lending<'a, ImpliedBound = &'a Self> {
    type Item;
}

/// The item type lent by `I` for a borrow of length `'a`.
pub type Item<'a, I> = <I as Lending<'a>>::Item;

pub trait LendingIterator: for<'a> Lending<'a> {
    fn next(&mut self) -> Option<Item<'_, Self>>;
}

pub struct WindowsMut<'s, T> {
    slice: &'s mut [T],
    size: usize,
    pos: usize,
}

pub fn windows_mut<T>(slice: &mut [T], size: usize) -> WindowsMut<'_, T> {
    assert!(size > 0, "window size must be positive");
    WindowsMut { slice, size, pos: 0 }
}

impl<'a, T> Lending<'a> for WindowsMut<'_, T> {
    type Item = &'a mut [T];
}

impl<T> LendingIterator for WindowsMut<'_, T> {
    // Reborrowing through `&mut self` is exactly what `Iterator` can't do and a lending
    // iterator can: the window lives only as long as this borrow of `self`.
    fn next(&mut self) -> Option<&mut [T]> {
        let window = self.slice.get_mut(self.pos..self.pos + self.size)?;
        self.pos += 1;
        Some(window)
    }
}

pub struct Lines<R> {
    reader: R,
    buf: String,
    done: bool,
}

pub fn lines<R: BufRead>(reader: R) -> Lines<R> {
    Lines { reader, buf: String::new(), done: false }
}

impl<'a, R> Lending<'a> for Lines<R> {
    type Item = io::Result<&'a str>;
}

impl<R: BufRead> LendingIterator for Lines<R> {
    fn next(&mut self) -> Option<io::Result<&str>> {
        if self.done {
            return None;
        }
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(0) => {
                self.done = true;
                None
            }
            Ok(_) => {
                let line = self.buf.strip_suffix('\n').unwrap_or(&self.buf);
                Some(Ok(line.strip_suffix('\r').unwrap_or(line)))
            }
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }
}

pub struct Enumerate<I> {
    inner: I,
    count: usize,
}

pub fn enumerate<I: LendingIterator>(inner: I) -> Enumerate<I> {
    Enumerate { inner, count: 0 }
}

impl<'a, I: LendingIterator> Lending<'a> for Enumerate<I> {
    type Item = (usize, Item<'a, I>);
}

impl<I: LendingIterator> LendingIterator for Enumerate<I> {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        let item = self.inner.next()?;
        let i = self.count;
        self.count += 1;
        Some((i, item))
    }
}

pub fn for_each<I, F>(mut it: I, mut f: F)
where
    I: LendingIterator,
    F: FnMut(Item<'_, I>),
{
    while let Some(item) = it.next() {
        f(item);
    }
}

pub fn count_where<I, P>(mut it: I, mut pred: P) -> usize
where
    I: LendingIterator,
    P: FnMut(&Item<'_, I>) -> bool,
{
    let mut n = 0;
    while let Some(item) = it.next() {
        if pred(&item) {
            n += 1;
        }
    }
    n
}
