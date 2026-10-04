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
    Pair {
        key: Cow<'a, str>,
        value: Cow<'a, str>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    EmptyKey { at: usize },
    UnterminatedQuote { at: usize },
    BadEscape { at: usize },
    TrailingGarbage { at: usize },
}

impl<'a> Field<'a> {
    // TODO(challenge): implement
    pub fn name(&self) -> &str {
        todo!("p03: name")
    }

    // TODO(challenge): implement
    pub fn value(&self) -> Option<&str> {
        todo!("p03: value")
    }

    // TODO(challenge): implement
    pub fn into_owned(self) -> Field<'static> {
        todo!("p03: into_owned")
    }
}

// TODO(challenge): implement
pub fn parse_fields(line: &str) -> Result<Vec<Field<'_>>, ParseError> {
    let _ = line;
    todo!("p03: parse_fields")
}

// TODO(challenge): implement
pub fn parse_owned(line: &str) -> Result<Vec<Field<'static>>, ParseError> {
    let _ = line;
    todo!("p03: parse_owned")
}
