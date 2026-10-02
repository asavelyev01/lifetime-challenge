//! HARNESS — do not edit.
use lifetimes_2026_10::p01_signatures::*;

#[test]
fn strip_examples() {
    assert_eq!(strip("v1.2", "v"), "1.2");
    assert_eq!(strip("1.2", "v"), "1.2");
}

#[test]
fn strip_edge_cases() {
    assert_eq!(strip("abc", ""), "abc", "empty prefix");
    assert_eq!(strip("", ""), "");
    assert_eq!(strip("", "x"), "");
    assert_eq!(strip("abc", "abc"), "", "prefix == whole string");
    assert_eq!(strip("ab", "abc"), "ab", "prefix longer than s");
    assert_eq!(strip("ééx", "é"), "éx", "strips only once, unicode");
    assert_eq!(strip("xab", "ab"), "xab", "only a prefix, not a substring");
}

#[test]
fn strip_returns_a_subslice_of_s() {
    let s = String::from("prefix-body");
    let out: &str = strip(&s, "prefix-");
    assert_eq!(out, "body");
    assert!(std::ptr::eq(out.as_ptr(), s[7..].as_ptr()), "should borrow from s, not copy");
}

#[test]
fn longest_examples() {
    assert_eq!(longest(&["ab", "ééé", "xyz"]), Some("ééé"));
    assert_eq!(longest(&[]), None);
}

#[test]
fn longest_counts_chars_and_prefers_first() {
    assert_eq!(longest(&["aa", "bb", "c"]), Some("aa"), "tie → first");
    assert_eq!(longest(&["éé", "abc"]), Some("abc"), "4 bytes but 2 chars");
    assert_eq!(longest(&[""]), Some(""));
    assert_eq!(longest(&["", "", "x", "y"]), Some("x"));
}

#[test]
fn lookup_examples() {
    assert_eq!(lookup("a = 1\nb=2=3", "b"), Some("2=3"));
    assert_eq!(lookup("a = 1\nb=2=3", "a"), Some("1"));
    assert_eq!(lookup("a = 1\nb=2=3", "c"), None);
}

#[test]
fn lookup_skips_comments_blank_and_malformed_lines() {
    let cfg = "\n# key = commented\n   \nnot a pair\n  key  =  value with spaces  \nkey=second\n";
    assert_eq!(lookup(cfg, "key"), Some("value with spaces"), "first match, trimmed");
    assert_eq!(lookup(cfg, "# key"), None, "comment lines are skipped entirely");
    assert_eq!(lookup(cfg, "not a pair"), None);
}

#[test]
fn lookup_details() {
    assert_eq!(lookup("Key=1", "key"), None, "case-sensitive");
    assert_eq!(lookup("k=", "k"), Some(""), "empty value");
    assert_eq!(lookup("=v", ""), Some("v"), "empty key is a key too");
    assert_eq!(lookup("k=1\r\nj=2\r\n", "k"), Some("1"), "CRLF is trimmed");
    assert_eq!(lookup("", "k"), None);
    assert_eq!(lookup("  # x=1\nx=2", "x"), Some("2"), "comment after trimming");
}

#[test]
fn plural_suffix_examples() {
    assert_eq!(plural_suffix("Box", 2), "es");
    assert_eq!(plural_suffix("cat", 0), "s");
}

#[test]
fn plural_suffix_all_rules() {
    for (w, want) in [
        ("bus", "es"), ("fox", "es"), ("quiz", "es"), ("church", "es"), ("wish", "es"),
        ("BUS", "es"), ("WiSh", "es"), ("cat", "s"), ("bath", "s"), ("", "s"), ("h", "s"),
        ("c", "s"), ("ß", "s"),
    ] {
        assert_eq!(plural_suffix(w, 2), want, "word {w:?}");
        assert_eq!(plural_suffix(w, 1), "", "word {w:?}, count 1");
    }
    assert_eq!(plural_suffix("box", usize::MAX), "es");
}
