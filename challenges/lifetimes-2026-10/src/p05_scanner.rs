//! # P05 — One lifetime is not enough       (difficulty: ★★★☆☆)
//!
//! `Scanner` finds whole-word keyword occurrences in a text. It borrows **two unrelated
//! things**: the keyword list and the text. The current definition squeezes both borrows
//! into a single `'a`, so spans can't outlive the keyword list, keywords can't outlive the
//! text, etc. (see `src/checks/p05.rs`).
//!
//! Redesign the type (and its impls) so that each output borrows only from what it really
//! comes from, then implement it. Keep the names `Scanner`, `Span`, `Scanner::new`,
//! `keyword_of`, `rest`, and `Iterator<Item = Span<..>>`; keep `Span`'s fields.
//!
//! ## Matching rules
//!
//! * Keywords are non-empty and consist only of *word chars* (`_` or `char::is_alphanumeric`).
//! * An occurrence of keyword `k` at byte offset `p` **matches** if the char before `p` (if
//!   any) and the char right after the occurrence (if any) are **not** word chars.
//! * Scan left to right. At each char position, among the keywords that match there, take the
//!   longest one (by byte length), ties → the lowest index in the keyword list. Emit it, and
//!   continue scanning right after it. Otherwise advance by one char. Matches never overlap.
//!
//! ## API
//!
//! * `Iterator` yields `Span { start, text, keyword }` — `start` is the byte offset in the
//!   text, `text` is the matched slice of the text, `keyword` the index in the keyword list.
//! * `keyword_of(&span)` — the keyword string for a span (`&keywords[span.keyword]`).
//! * `rest()` — the text after the end of the last emitted span (initially the whole text);
//!   `""` once the iterator has returned `None`.
//! * `highlight(keywords, text)` — the text with every match wrapped in `[` `]`.
//!
//! ## Example
//!
//! keywords `["if", "else", "else_if"]`, text `"if x {} else_if y {} elsewhere else"` →
//! spans at 0 (`if`, kw 0), 8 (`else_if`, kw 2), 31 (`else`, kw 1). `"elsewhere"` doesn't
//! match (followed by a word char). `highlight` →
//! `"[if] x {} [else_if] y {} elsewhere [else]"`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<'a> {
    pub start: usize,
    pub text: &'a str,
    pub keyword: usize,
}

// TODO(challenge): redesign the lifetimes
pub struct Scanner<'a> {
    keywords: &'a [String],
    text: &'a str,
}

// TODO(challenge): redesign the lifetimes + implement
impl<'a> Scanner<'a> {
    pub fn new(keywords: &'a [String], text: &'a str) -> Self {
        Scanner { keywords, text }
    }

    pub fn keyword_of(&self, span: &Span<'_>) -> &str {
        let _ = span;
        todo!("p05: keyword_of")
    }

    pub fn rest(&self) -> &str {
        todo!("p05: rest")
    }
}

// TODO(challenge): redesign the lifetimes + implement
impl<'a> Iterator for Scanner<'a> {
    type Item = Span<'a>;

    fn next(&mut self) -> Option<Span<'a>> {
        let _ = (self.keywords, self.text);
        todo!("p05: next")
    }
}

// TODO(challenge): implement
pub fn highlight(keywords: &[String], text: &str) -> String {
    let _ = (keywords, text);
    todo!("p05: highlight")
}
