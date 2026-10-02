//! HARNESS — do not edit.
use lifetimes_2026_10::p03_fields::*;
use std::borrow::Cow;

fn flag(s: &str) -> Field<'_> {
    Field::Flag(Cow::Borrowed(s))
}
fn pair<'a>(k: &'a str, v: &'a str) -> Field<'a> {
    Field::Pair { key: Cow::Borrowed(k), value: Cow::Borrowed(v) }
}

/// Which Cows are borrowed: one bool per Cow, in field order (Flag: [name], Pair: [key, value]).
fn borrowedness(fields: &[Field<'_>]) -> Vec<bool> {
    let b = |c: &Cow<'_, str>| matches!(c, Cow::Borrowed(_));
    fields
        .iter()
        .flat_map(|f| match f {
            Field::Flag(n) => vec![b(n)],
            Field::Pair { key, value } => vec![b(key), b(value)],
        })
        .collect()
}

#[test]
fn statement_example() {
    let line = r#"verbose level=3 msg="hi \"you\"" path="/tmp""#;
    let fields = parse_fields(line).unwrap();
    assert_eq!(fields, [flag("verbose"), pair("level", "3"), pair("msg", "hi \"you\""), pair("path", "/tmp")]);
    assert_eq!(borrowedness(&fields), [true, true, true, true, false, true, true]);
    assert_eq!(parse_fields("a ==b"), Err(ParseError::EmptyKey { at: 2 }));
}

#[test]
fn empty_and_blank_lines() {
    assert_eq!(parse_fields("").unwrap(), []);
    assert_eq!(parse_fields(" \t  ").unwrap(), []);
}

#[test]
fn flags_and_bare_values() {
    assert_eq!(parse_fields("  a\tb  ").unwrap(), [flag("a"), flag("b")]);
    assert_eq!(parse_fields(r#"x"y"#).unwrap(), [flag(r#"x"y"#)], "quotes are only special after =");
    assert_eq!(parse_fields("k=").unwrap(), [pair("k", "")], "empty bare value");
    assert_eq!(parse_fields("k= j").unwrap(), [pair("k", ""), flag("j")]);
    assert_eq!(parse_fields("k=a=b").unwrap(), [pair("k", "a=b")], "split at the first =");
    assert_eq!(parse_fields(r#"k=a"b"#).unwrap(), [pair("k", r#"a"b"#)], "quote inside bare value");
    assert_eq!(parse_fields("ключ=значение ✓").unwrap(), [pair("ключ", "значение"), flag("✓")]);
}

#[test]
fn quoted_values() {
    assert_eq!(parse_fields(r#"k="""#).unwrap(), [pair("k", "")]);
    assert_eq!(parse_fields(r#"k="a b  c" d"#).unwrap(), [pair("k", "a b  c"), flag("d")]);
    assert_eq!(parse_fields(r#"k="a=b""#).unwrap(), [pair("k", "a=b")]);
    let f = parse_fields(r#"k="1\\2\n3\"""#).unwrap();
    assert_eq!(f, [pair("k", "1\\2\n3\"")]);
    assert_eq!(borrowedness(&f), [true, false], "escaped value is owned, key still borrowed");
}

#[test]
fn everything_unescaped_is_borrowed_from_the_line() {
    let line = String::from(r#"f k=v q="quoted" e="""#);
    let fields = parse_fields(&line).unwrap();
    assert!(borrowedness(&fields).iter().all(|&b| b), "{fields:?}");
    let range = line.as_ptr() as usize..line.as_ptr() as usize + line.len() + 1;
    for f in &fields {
        assert!(range.contains(&(f.name().as_ptr() as usize)), "{f:?} name not from the line");
    }
}

#[test]
fn errors() {
    use ParseError::*;
    assert_eq!(parse_fields("=v"), Err(EmptyKey { at: 0 }));
    assert_eq!(parse_fields("ok  =v"), Err(EmptyKey { at: 4 }));
    assert_eq!(parse_fields(r#"a k="open"#), Err(UnterminatedQuote { at: 4 }));
    assert_eq!(parse_fields(r#"k=""#), Err(UnterminatedQuote { at: 2 }));
    assert_eq!(parse_fields(r#"k="a\tb""#), Err(BadEscape { at: 4 }));
    assert_eq!(parse_fields(r#"k="ab\"#), Err(BadEscape { at: 5 }), "backslash at the end");
    assert_eq!(parse_fields(r#"k="ab"c"#), Err(TrailingGarbage { at: 6 }));
    assert_eq!(parse_fields(r#"k="ab""x""#), Err(TrailingGarbage { at: 6 }));
    assert_eq!(parse_fields(r#"é k="\q" =x"#), Err(BadEscape { at: 6 }), "first error wins, byte offsets");
}

#[test]
fn accessors() {
    let fields = parse_fields(r#"flag k="v\n""#).unwrap();
    assert_eq!(fields[0].name(), "flag");
    assert_eq!(fields[0].value(), None);
    assert_eq!(fields[1].name(), "k");
    assert_eq!(fields[1].value(), Some("v\n"));
}

#[test]
fn into_owned_preserves_content_and_owns_everything() {
    let line = String::from(r#"a b=c d="e\"f""#);
    let borrowed = parse_fields(&line).unwrap();
    let owned: Vec<Field<'static>> = borrowed.iter().cloned().map(Field::into_owned).collect();
    assert_eq!(owned, borrowed);
    assert!(borrowedness(&owned).iter().all(|&b| !b), "into_owned must not keep borrows: {owned:?}");
}

#[test]
fn parse_owned_matches_parse_fields() {
    for line in ["", "a", r#"a=1 b="2 3" c"#, r#"x="\\""#, "=bad", r#"k="open"#] {
        let owned = parse_owned(line);
        let borrowed = parse_fields(line);
        assert_eq!(owned, borrowed, "line {line:?}");
    }
}
