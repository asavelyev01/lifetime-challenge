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

// `'k` = the keyword list, `'t` = the text. Every output borrows from exactly one of them.
pub struct Scanner<'k, 't> {
    keywords: &'k [String],
    text: &'t str,
    pos: usize,
}

fn is_word(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

impl<'k, 't> Scanner<'k, 't> {
    pub fn new(keywords: &'k [String], text: &'t str) -> Self {
        Scanner { keywords, text, pos: 0 }
    }

    pub fn keyword_of(&self, span: &Span<'_>) -> &'k str {
        &self.keywords[span.keyword]
    }

    pub fn rest(&self) -> &'t str {
        &self.text[self.pos..]
    }

    /// The best keyword matching at byte offset `p` (assumed to be a word start).
    fn match_at(&self, p: usize) -> Option<(usize, &'k str)> {
        let tail = &self.text[p..];
        self.keywords
            .iter()
            .enumerate()
            .filter(|(_, k)| tail.starts_with(k.as_str()) && !tail[k.len()..].starts_with(is_word))
            // longest wins; on equal length, `rev` + `max_by_key` (last max) keeps the lowest index
            .rev()
            .max_by_key(|(_, k)| k.len())
            .map(|(i, k)| (i, k.as_str()))
    }
}

impl<'k, 't> Iterator for Scanner<'k, 't> {
    type Item = Span<'t>;

    fn next(&mut self) -> Option<Span<'t>> {
        let text = self.text;
        let mut prev_is_word = text[..self.pos].chars().next_back().is_some_and(is_word);
        for (off, c) in text[self.pos..].char_indices() {
            let p = self.pos + off;
            if !prev_is_word && let Some((keyword, k)) = self.match_at(p) {
                self.pos = p + k.len();
                return Some(Span { start: p, text: &text[p..self.pos], keyword });
            }
            prev_is_word = is_word(c);
        }
        self.pos = text.len();
        None
    }
}

pub fn highlight(keywords: &[String], text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for span in Scanner::new(keywords, text) {
        out.push_str(&text[last..span.start]);
        out.push('[');
        out.push_str(span.text);
        out.push(']');
        last = span.start + span.text.len();
    }
    out.push_str(&text[last..]);
    out
}
