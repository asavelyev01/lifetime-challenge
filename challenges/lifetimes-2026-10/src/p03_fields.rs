//! # P03 — Borrow when you can, own when you must      (difficulty: ★★☆☆☆)
//!
//! Parse a log-style line of fields into `Field<'a>`s that borrow from the line whenever
//! possible, and convert them into `Field<'static>` when they have to outlive it.
//!
//! ## Grammar
//!
//! * Fields are separated by one or more ASCII whitespace chars (`char::is_ascii_whitespace`)
//!   outside of quotes. Leading/trailing whitespace is fine. An empty line has no fields.
//! * A field starts at a non-whitespace char. Scan forward until `'='`, whitespace or the end:
//!   * reaching whitespace/end first → `Flag(the scanned text)` (it may contain `"`);
//!   * reaching `'='` first → the scanned text is the **key** (`EmptyKey { at }` if it is
//!     empty, `at` = byte offset of the field start), and a value follows:
//!     * if the char after `=` is `"`, the value is **quoted**: it runs until the next
//!       unescaped `"`. Escapes: `\"` → `"`, `\\` → `\`, `\n` → newline. A `\` followed by
//!       anything else (or by the end of the line) → `BadEscape { at }` (offset of the `\`).
//!       No closing quote → `UnterminatedQuote { at }` (offset of the opening `"`).
//!       After the closing quote the next char must be whitespace or the end of the line,
//!       otherwise `TrailingGarbage { at }` (offset of that char).
//!     * otherwise the value is **bare**: everything up to the next whitespace/end. It may be
//!       empty and may contain `=` and `"`.
//! * The first error found scanning left to right is returned.
//!
//! ## Borrowing requirements
//!
//! Every `Cow` produced by `parse_fields` must be `Cow::Borrowed` (a slice of the input)
//! **unless** it is a quoted value that actually contained an escape — only then is it
//! `Cow::Owned`. (Tests check this.)
//!
//! ## Examples
//!
//! `r#"verbose level=3 msg="hi \"you\"" path="/tmp""#` →
//! `Flag("verbose")`, `Pair{level, "3"}`, `Pair{msg, "hi \"you\""}` (owned),
//! `Pair{path, "/tmp"}` (borrowed — no escapes).
//!
//! `"a ==b"` → `Flag("a")`, then `EmptyKey { at: 2 }`.
//!
//! ## API
//!
//! * `Field::name()` — the flag's name or the pair's key; `Field::value()` — `None` for flags.
//! * `Field::into_owned()` — same field, but no longer borrowing anything.
//! * `parse_owned(line)` — like `parse_fields`, but the result outlives `line`.

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field<'a> {
    Flag(Cow<'a, str>),
    Pair { key: Cow<'a, str>, value: Cow<'a, str> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    EmptyKey { at: usize },
    UnterminatedQuote { at: usize },
    BadEscape { at: usize },
    TrailingGarbage { at: usize },
}

impl<'a> Field<'a> {
    pub fn name(&self) -> &str {
        match self {
            Field::Flag(name) | Field::Pair { key: name, .. } => name,
        }
    }

    pub fn value(&self) -> Option<&str> {
        match self {
            Field::Flag(_) => None,
            Field::Pair { value, .. } => Some(value),
        }
    }

    /// `Cow::into_owned` gives a `String`; wrapping it back in `Cow::Owned` produces a
    /// `Cow<'b, str>` for *any* `'b`, in particular `'static`.
    pub fn into_owned(self) -> Field<'static> {
        let own = |c: Cow<'_, str>| Cow::Owned(c.into_owned());
        match self {
            Field::Flag(name) => Field::Flag(own(name)),
            Field::Pair { key, value } => Field::Pair { key: own(key), value: own(value) },
        }
    }
}

pub fn parse_fields(line: &str) -> Result<Vec<Field<'_>>, ParseError> {
    let bytes = line.as_bytes();
    let is_ws = |i: usize| bytes[i].is_ascii_whitespace();
    let mut fields = Vec::new();
    let mut i = 0;
    loop {
        while i < bytes.len() && is_ws(i) {
            i += 1;
        }
        if i == bytes.len() {
            return Ok(fields);
        }
        // All delimiters are ASCII, so byte-wise scanning stays on char boundaries.
        let start = i;
        while i < bytes.len() && !is_ws(i) && bytes[i] != b'=' {
            i += 1;
        }
        if i == bytes.len() || is_ws(i) {
            fields.push(Field::Flag(Cow::Borrowed(&line[start..i])));
            continue;
        }
        if i == start {
            return Err(ParseError::EmptyKey { at: start });
        }
        let key = Cow::Borrowed(&line[start..i]);
        i += 1; // '='
        let value = if bytes.get(i) == Some(&b'"') {
            let (value, end) = parse_quoted(line, i)?;
            if end < bytes.len() && !is_ws(end) {
                return Err(ParseError::TrailingGarbage { at: end });
            }
            i = end;
            value
        } else {
            let v_start = i;
            while i < bytes.len() && !is_ws(i) {
                i += 1;
            }
            Cow::Borrowed(&line[v_start..i])
        };
        fields.push(Field::Pair { key, value });
    }
}

/// Parses a quoted value whose opening quote is at `open`. Returns the value and the offset
/// just past the closing quote. Allocates only when an escape is present.
fn parse_quoted(line: &str, open: usize) -> Result<(Cow<'_, str>, usize), ParseError> {
    let bytes = line.as_bytes();
    let body = open + 1;
    let mut owned: Option<String> = None; // Some once we've seen an escape
    let mut chunk = body; // start of the not-yet-copied run
    let mut i = body;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                let value = match owned {
                    None => Cow::Borrowed(&line[body..i]),
                    Some(mut s) => {
                        s.push_str(&line[chunk..i]);
                        Cow::Owned(s)
                    }
                };
                return Ok((value, i + 1));
            }
            b'\\' => {
                let unescaped = match bytes.get(i + 1) {
                    Some(b'"') => '"',
                    Some(b'\\') => '\\',
                    Some(b'n') => '\n',
                    _ => return Err(ParseError::BadEscape { at: i }),
                };
                let s = owned.get_or_insert_with(String::new);
                s.push_str(&line[chunk..i]);
                s.push(unescaped);
                i += 2;
                chunk = i;
            }
            _ => i += 1,
        }
    }
    Err(ParseError::UnterminatedQuote { at: open })
}

pub fn parse_owned(line: &str) -> Result<Vec<Field<'static>>, ParseError> {
    Ok(parse_fields(line)?.into_iter().map(Field::into_owned).collect())
}
