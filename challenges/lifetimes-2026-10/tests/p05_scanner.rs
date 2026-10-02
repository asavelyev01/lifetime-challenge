//! HARNESS — do not edit.
use lifetimes_2026_10::p05_scanner::*;

fn kws(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

fn scan(keywords: &[&str], text: &str) -> Vec<(usize, String, usize)> {
    let keywords = kws(keywords);
    Scanner::new(&keywords, text).map(|s| (s.start, s.text.to_string(), s.keyword)).collect()
}

fn t(items: &[(usize, &str, usize)]) -> Vec<(usize, String, usize)> {
    items.iter().map(|&(a, b, c)| (a, b.to_string(), c)).collect()
}

/// Brute-force oracle straight from the spec.
fn oracle(keywords: &[&str], text: &str) -> Vec<(usize, String, usize)> {
    let word = |c: char| c == '_' || c.is_alphanumeric();
    let mut out = vec![];
    let mut p = 0;
    while p < text.len() {
        let before_ok = text[..p].chars().next_back().is_none_or(|c| !word(c));
        let mut best: Option<(usize, usize)> = None; // (len, idx)
        if before_ok {
            for (i, k) in keywords.iter().enumerate() {
                if text[p..].starts_with(k)
                    && text[p + k.len()..].chars().next().is_none_or(|c| !word(c))
                    && best.is_none_or(|(l, _)| k.len() > l)
                {
                    best = Some((k.len(), i));
                }
            }
        }
        match best {
            Some((l, i)) => {
                out.push((p, text[p..p + l].to_string(), i));
                p += l;
            }
            None => p += text[p..].chars().next().unwrap().len_utf8(),
        }
    }
    out
}

#[test]
fn statement_example() {
    let k = ["if", "else", "else_if"];
    let text = "if x {} else_if y {} elsewhere else";
    assert_eq!(scan(&k, text), t(&[(0, "if", 0), (8, "else_if", 2), (31, "else", 1)]));
    assert_eq!(highlight(&kws(&k), text), "[if] x {} [else_if] y {} elsewhere [else]");
}

#[test]
fn whole_words_only() {
    let k = ["in"];
    assert_eq!(scan(&k, "in inside within in_ _in in"), t(&[(0, "in", 0), (25, "in", 0)]));
    assert_eq!(scan(&k, "in(in)in"), t(&[(0, "in", 0), (3, "in", 0), (6, "in", 0)]));
    assert_eq!(scan(&k, "éin in2 in"), t(&[(9, "in", 0)]), "unicode letters/digits are word chars");
}

#[test]
fn longest_then_lowest_index() {
    assert_eq!(scan(&["a", "a_b"], "a_b a"), t(&[(0, "a_b", 1), (4, "a", 0)]));
    assert_eq!(scan(&["x", "x"], "x"), t(&[(0, "x", 0)]), "duplicate keyword → lowest index");
}

#[test]
fn empty_inputs() {
    assert_eq!(scan(&[], "anything here"), t(&[]));
    assert_eq!(scan(&["a"], ""), t(&[]));
}

#[test]
fn spans_borrow_from_the_text() {
    let keywords = kws(&["fn"]);
    let text = String::from("pub fn x");
    let span = Scanner::new(&keywords, &text).next().unwrap();
    assert_eq!(span.text.as_ptr(), text[4..].as_ptr(), "span text must be a slice of the text");
}

#[test]
fn keyword_of_and_rest() {
    let keywords = kws(&["let", "mut"]);
    let text = "let mut x";
    let mut sc = Scanner::new(&keywords, text);
    assert_eq!(sc.rest(), text, "initially the whole text");
    let s1 = sc.next().unwrap();
    assert_eq!(sc.keyword_of(&s1), "let");
    assert_eq!(sc.rest(), " mut x");
    let s2 = sc.next().unwrap();
    assert_eq!(sc.keyword_of(&s2), "mut");
    assert_eq!(sc.rest(), " x");
    assert_eq!(sc.next(), None);
    assert_eq!(sc.rest(), "");
    assert_eq!(sc.next(), None, "fused");
}

#[test]
fn highlight_cases() {
    assert_eq!(highlight(&kws(&["a"]), ""), "");
    assert_eq!(highlight(&kws(&["a"]), "b"), "b");
    assert_eq!(highlight(&kws(&["ü"]), "ü,ü"), "[ü],[ü]");
}

#[test]
fn randomized_against_oracle() {
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    let mut rng = SmallRng::seed_from_u64(5);
    let alphabet = ['a', 'b', '_', ' ', '.', 'é', '1'];
    let pool = ["a", "b", "ab", "a_b", "é", "ba", "a1", "b"];
    for _ in 0..2000 {
        let n = rng.random_range(0..4);
        let k: Vec<&str> = (0..n).map(|_| pool[rng.random_range(0..pool.len())]).collect();
        let text: String = (0..rng.random_range(0..20)).map(|_| alphabet[rng.random_range(0..alphabet.len())]).collect();
        assert_eq!(scan(&k, &text), oracle(&k, &text), "keywords {k:?}, text {text:?}");
    }
}
