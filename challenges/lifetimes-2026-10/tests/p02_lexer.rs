//! HARNESS — do not edit.
use lifetimes_2026_10::p02_lexer::*;
use Token::*;

fn lex(src: &str) -> Vec<Result<Token<'_>, LexError>> {
    Lexer::new(src).collect()
}

#[test]
fn statement_example() {
    let src = "let x=\"hi\" ;";
    assert_eq!(
        tokenize(src).unwrap(),
        [Ident("let"), Ident("x"), Punct('='), Str("hi"), Punct(';')]
    );
    let mut lx = Lexer::new(src);
    assert_eq!(lx.remainder(), src, "initially the whole source");
    assert_eq!(lx.offset(), 0);
    lx.next();
    assert_eq!(lx.remainder(), " x=\"hi\" ;");
    assert_eq!(lx.offset(), 3);
}

#[test]
fn empty_and_whitespace_only() {
    assert!(lex("").is_empty());
    assert!(lex(" \t\n\r\u{3000}").is_empty(), "unicode whitespace is skipped too");
    let mut lx = Lexer::new("   ");
    assert_eq!(lx.next(), None);
    assert_eq!(lx.remainder(), "");
    assert_eq!(lx.offset(), 3);
}

#[test]
fn identifiers() {
    assert_eq!(tokenize("_ _a a_1 größe_2").unwrap(), [Ident("_"), Ident("_a"), Ident("a_1"), Ident("größe_2")]);
    assert_eq!(tokenize("αβγ").unwrap(), [Ident("αβγ")]);
}

#[test]
fn numbers_are_ascii_digit_runs() {
    assert_eq!(tokenize("123abc").unwrap(), [Number("123"), Ident("abc")]);
    assert_eq!(tokenize("abc123").unwrap(), [Ident("abc123")]);
    assert_eq!(
        tokenize("99999999999999999999999999 0").unwrap(),
        [Number("99999999999999999999999999"), Number("0")],
        "no overflow, raw text"
    );
    assert_eq!(tokenize("1.5").unwrap(), [Number("1"), Punct('.'), Number("5")]);
}

#[test]
fn strings() {
    assert_eq!(tokenize(r#""""#).unwrap(), [Str("")]);
    assert_eq!(tokenize("\"a b\nc\"x").unwrap(), [Str("a b\nc"), Ident("x")]);
    assert_eq!(tokenize(r#""a""b""#).unwrap(), [Str("a"), Str("b")]);
    assert_eq!(tokenize(r#""héllo wörld""#).unwrap(), [Str("héllo wörld")]);
}

#[test]
fn punctuation_is_single_chars() {
    assert_eq!(tokenize("+=").unwrap(), [Punct('+'), Punct('=')]);
    assert_eq!(tokenize("→🦀").unwrap(), [Punct('→'), Punct('🦀')]);
}

#[test]
fn unterminated_string_errors_once_then_stops() {
    let src = "ok \"never closed";
    let mut lx = Lexer::new(src);
    assert_eq!(lx.next(), Some(Ok(Ident("ok"))));
    assert_eq!(lx.next(), Some(Err(LexError::UnterminatedString { start: 3 })));
    assert_eq!(lx.next(), None, "exhausted after an error");
    assert_eq!(lx.next(), None);
    assert_eq!(lx.remainder(), "");
    assert_eq!(lx.offset(), src.len());
}

#[test]
fn tokenize_returns_first_error() {
    assert_eq!(tokenize("é \"x"), Err(LexError::UnterminatedString { start: 3 }), "byte offset");
    assert_eq!(tokenize("\""), Err(LexError::UnterminatedString { start: 0 }));
}

#[test]
fn remainder_and_offset_track_progress() {
    let src = "  foo  (bar)  ";
    let mut lx = Lexer::new(src);
    let mut seen = vec![];
    while let Some(t) = lx.next() {
        seen.push((t.unwrap(), lx.offset(), lx.remainder().to_string()));
        assert_eq!(&src[lx.offset()..], lx.remainder());
    }
    assert_eq!(
        seen,
        [
            (Ident("foo"), 5, "  (bar)  ".to_string()),
            (Punct('('), 8, "bar)  ".to_string()),
            (Ident("bar"), 11, ")  ".to_string()),
            (Punct(')'), 12, "  ".to_string()),
        ]
    );
    assert_eq!(lx.remainder(), "");
}

#[test]
fn tokens_point_into_the_source() {
    let src = String::from("word 77 \"str\"");
    let toks = tokenize(&src).unwrap();
    let ptrs: Vec<usize> = toks.iter().map(|t| t.lexeme().unwrap().as_ptr() as usize - src.as_ptr() as usize).collect();
    assert_eq!(ptrs, [0, 5, 9], "lexemes must be slices of the source");
}

#[test]
fn lexeme_per_variant() {
    assert_eq!(Ident("a").lexeme(), Some("a"));
    assert_eq!(Number("1").lexeme(), Some("1"));
    assert_eq!(Str("").lexeme(), Some(""));
    assert_eq!(Punct('+').lexeme(), None);
}

#[test]
fn randomized_round_trip() {
    // Build random sources from known tokens and check that lexing recovers them.
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    let mut rng = SmallRng::seed_from_u64(0x1ec5);
    let idents = ["a", "_x", "foo_1", "ünï", "z9"];
    let nums = ["0", "42", "007"];
    let strs = ["\"\"", "\"a b\"", "\"✓\""];
    let puncts = ["+", "(", ")", "→", ";"];
    for _ in 0..300 {
        let mut src = String::new();
        let mut want = vec![];
        let mut prev_word = false;
        for _ in 0..rng.random_range(0..12) {
            let kind = rng.random_range(0..4);
            let (txt, tok, word) = match kind {
                0 => { let s = idents[rng.random_range(0..idents.len())]; (s, Ident(s), true) }
                1 => { let s = nums[rng.random_range(0..nums.len())]; (s, Number(s), true) }
                2 => { let s = strs[rng.random_range(0..strs.len())]; (s, Str(&s[1..s.len() - 1]), false) }
                _ => { let s = puncts[rng.random_range(0..puncts.len())]; (s, Punct(s.chars().next().unwrap()), false) }
            };
            if (prev_word && word) || rng.random_bool(0.5) {
                src.push_str([" ", "\t", "\n  "][rng.random_range(0..3)]);
            }
            src.push_str(txt);
            want.push(tok);
            prev_word = word;
        }
        assert_eq!(tokenize(&src).unwrap(), want, "source {src:?}");
    }
}
