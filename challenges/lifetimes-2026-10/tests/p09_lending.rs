//! HARNESS — do not edit.
//!
//! Only uses `while let … next()` and `'static` iterators with `for_each`/`count_where`, so
//! it compiles against both the stub and a redesigned trait. Borrowed iterators passed to
//! `for_each`/`count_where` are covered by `src/checks/p09.rs`.
use lifetimes_2026_10::p09_lending::*;
use std::io::{self, BufRead, Cursor, Read};

fn collect_windows(v: &mut [i32], size: usize) -> Vec<Vec<i32>> {
    let mut it = windows_mut(v, size);
    let mut out = vec![];
    while let Some(w) = it.next() {
        out.push(w.to_vec());
    }
    out
}

fn collect_lines<R: BufRead>(r: R) -> Vec<Result<String, io::ErrorKind>> {
    let mut it = lines(r);
    let mut out = vec![];
    while let Some(l) = it.next() {
        out.push(l.map(str::to_string).map_err(|e| e.kind()));
    }
    out
}

fn ok(lines: &[&str]) -> Vec<Result<String, io::ErrorKind>> {
    lines.iter().map(|l| Ok(l.to_string())).collect()
}

#[test]
fn windows_basic() {
    let mut v = [1, 2, 3, 4];
    assert_eq!(collect_windows(&mut v, 2), [[1, 2], [2, 3], [3, 4]]);
    assert_eq!(collect_windows(&mut v, 4), [[1, 2, 3, 4]]);
    assert_eq!(collect_windows(&mut v, 1), [[1], [2], [3], [4]]);
}

#[test]
fn windows_larger_than_slice_or_empty() {
    let mut v = [1, 2];
    assert!(collect_windows(&mut v, 3).is_empty());
    assert!(collect_windows(&mut [], 1).is_empty());
}

#[test]
#[should_panic]
fn windows_of_size_zero_panic() {
    let mut v = [1];
    let _ = windows_mut(&mut v, 0);
}

#[test]
fn windows_are_mutable_and_see_earlier_writes() {
    let mut v = vec![1, 1, 0, 0, 0, 0, 0, 0];
    let mut it = windows_mut(&mut v, 3);
    while let Some(w) = it.next() {
        w[2] = w[0] + w[1]; // fibonacci in place
    }
    assert_eq!(v, [1, 1, 2, 3, 5, 8, 13, 21]);
}

#[test]
fn windows_are_fused() {
    let mut v = [1, 2];
    let mut it = windows_mut(&mut v, 2);
    assert!(it.next().is_some());
    assert!(it.next().is_none());
    assert!(it.next().is_none());
}

#[test]
fn lines_statement_example() {
    assert_eq!(collect_lines("a\r\n\nb".as_bytes()), ok(&["a", "", "b"]));
}

#[test]
fn lines_edge_cases() {
    assert_eq!(collect_lines("".as_bytes()), ok(&[]));
    assert_eq!(collect_lines("\n".as_bytes()), ok(&[""]));
    assert_eq!(collect_lines("x\n".as_bytes()), ok(&["x"]), "no extra empty line at the end");
    assert_eq!(collect_lines("x\n\n".as_bytes()), ok(&["x", ""]));
    assert_eq!(collect_lines("a\r\r\nb\r".as_bytes()), ok(&["a\r", "b"]), "one \\r only, also on the last line");
    assert_eq!(collect_lines("ünï\r\ncödé".as_bytes()), ok(&["ünï", "cödé"]));
}

#[test]
fn lines_reuse_one_buffer() {
    let text = "first line\nsecond\nthird line here\n".repeat(50);
    let mut it = lines(Cursor::new(text.into_bytes()));
    let mut ptrs = std::collections::HashSet::new();
    let mut n = 0;
    while let Some(l) = it.next() {
        ptrs.insert(l.unwrap().as_ptr() as usize);
        n += 1;
    }
    assert_eq!(n, 150);
    assert!(ptrs.len() <= 3, "lines must be lent from a reused buffer ({} distinct allocations)", ptrs.len());
}

#[test]
fn lines_invalid_utf8_errors_once_then_stops() {
    let bytes: Vec<u8> = b"ok\n\xff\xfe\nnever\n".to_vec();
    assert_eq!(collect_lines(Cursor::new(bytes)), [Ok("ok".to_string()), Err(io::ErrorKind::InvalidData)]);
}

struct Failing {
    data: Cursor<Vec<u8>>,
}
impl Read for Failing {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.data.read(buf)? {
            0 => Err(io::Error::new(io::ErrorKind::ConnectionReset, "boom")),
            n => Ok(n),
        }
    }
}

#[test]
fn lines_io_error_errors_once_then_stops() {
    let r = io::BufReader::with_capacity(4, Failing { data: Cursor::new(b"ab\ncd".to_vec()) });
    assert_eq!(collect_lines(r), [Ok("ab".to_string()), Err(io::ErrorKind::ConnectionReset)]);
}

#[test]
fn enumerate_counts_from_zero() {
    let mut it = enumerate(lines(Cursor::new(b"x\ny\nz".to_vec())));
    let mut out = vec![];
    while let Some((i, l)) = it.next() {
        out.push(format!("{i}:{}", l.unwrap()));
    }
    assert_eq!(out, ["0:x", "1:y", "2:z"]);
}

#[test]
fn enumerate_over_windows() {
    let mut v = [0; 4];
    let mut it = enumerate(windows_mut(&mut v, 2));
    while let Some((i, w)) = it.next() {
        w[1] += i as i32 + 1;
    }
    assert_eq!(v, [0, 1, 2, 3]);
}

#[test]
fn for_each_and_count_where_on_owned_iterators() {
    let src = || lines(Cursor::new(b"alpha\n\nbeta\ngamma\n".to_vec()));
    let mut seen = vec![];
    for_each(src(), |l| seen.push(l.unwrap().to_uppercase()));
    assert_eq!(seen, ["ALPHA", "", "BETA", "GAMMA"]);
    assert_eq!(count_where(src(), |l| l.as_ref().is_ok_and(|s| s.contains('a'))), 3);
    assert_eq!(count_where(src(), |_| false), 0);
    assert_eq!(count_where(lines(Cursor::new(Vec::new())), |_| true), 0);
    let mut idx = vec![];
    for_each(enumerate(src()), |(i, l)| if l.unwrap().is_empty() { idx.push(i) });
    assert_eq!(idx, [1]);
}
