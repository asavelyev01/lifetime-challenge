//! # P02 — A zero-copy lexer               (difficulty: ★☆☆☆☆)
//!
//! `Lexer<'a>` walks over a borrowed `&'a str` and yields [`Token<'a>`]s that borrow from
//! the **source**, not from the lexer. Tokens, lexemes and the remainder must stay usable
//! after the lexer itself is gone (see `src/checks/p02.rs`).
//!
//! ## Lexing rules
//!
//! Starting at the current position:
//! 1. Skip whitespace (`char::is_whitespace`).
//! 2. `Ident`  — first char is `_` or `char::is_alphabetic`; continues while chars are `_` or
//!    `char::is_alphanumeric`. Unicode is fine: `"größe_2"` is one identifier.
//! 3. `Number` — a maximal run of ASCII digits (`0-9`). The raw text is kept, so there is no
//!    overflow: `"123abc"` → `Number("123"), Ident("abc")`.
//! 4. `Str`    — `"` … next `"`; the payload excludes the quotes; there are no escapes and the
//!    payload may contain anything (including newlines). An unterminated string yields
//!    `Err(LexError::UnterminatedString { start })` where `start` is the byte offset of the
//!    opening quote; after that the lexer is exhausted (returns `None` forever).
//! 5. Anything else is a single-char `Punct(c)`.
//!
//! ## API
//!
//! * `Iterator for Lexer<'a>` with `Item = Result<Token<'a>, LexError>`.
//! * `remainder()` — the not-yet-consumed source: everything after the last token yielded
//!   (leading whitespace is **not** skipped). Initially the whole source. Once the lexer is
//!   exhausted or has produced an error, `""`.
//! * `offset()` — the byte offset of `remainder()` in the source
//!   (so `src[lexer.offset()..] == *lexer.remainder()`).
//! * `Token::lexeme()` — the borrowed text of `Ident`/`Number`/`Str` (for `Str` the payload
//!   without quotes); `None` for `Punct`.
//! * `tokenize(src)` — all tokens, or the first error.
//!
//! ## Example
//!
//! `"let x=\"hi\" ;"` → `Ident("let"), Ident("x"), Punct('='), Str("hi"), Punct(';')`.
//! After the first `next()`, `remainder() == " x=\"hi\" ;"` and `offset() == 3`.
//!
//! You may change the bodies, add private fields/helpers, and fix the signatures marked
//! `TODO(challenge)`. Don't change the `Token`/`LexError` definitions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Ident(&'a str),
    Number(&'a str),
    Str(&'a str),
    Punct(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexError {
    UnterminatedString { start: usize },
}

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer { src, pos: 0 }
    }

    pub fn offset(&self) -> usize {
        self.pos
    }

    // `self.src` is a `&'a str` (Copy), so slicing it yields `&'a str` — not tied to `&self`.
    pub fn remainder(&self) -> &'a str {
        &self.src[self.pos..]
    }

    /// Byte offset of the first char at/after `from` that fails `pred` (or the end).
    fn scan_while(&self, from: usize, pred: impl Fn(char) -> bool) -> usize {
        self.src[from..].find(|c| !pred(c)).map_or(self.src.len(), |i| from + i)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.scan_while(self.pos, char::is_whitespace);
        let Some(c) = self.src[start..].chars().next() else {
            self.pos = start; // only whitespace was left: exhausted
            return None;
        };
        let src = self.src; // copy the `&'a str` out so the slices below are `&'a str`
        let (token, end) = if c == '_' || c.is_alphabetic() {
            let end = self.scan_while(start, |c| c == '_' || c.is_alphanumeric());
            (Token::Ident(&src[start..end]), end)
        } else if c.is_ascii_digit() {
            let end = self.scan_while(start, |c| c.is_ascii_digit());
            (Token::Number(&src[start..end]), end)
        } else if c == '"' {
            match src[start + 1..].find('"') {
                Some(len) => (Token::Str(&src[start + 1..start + 1 + len]), start + len + 2),
                None => {
                    self.pos = src.len();
                    return Some(Err(LexError::UnterminatedString { start }));
                }
            }
        } else {
            (Token::Punct(c), start + c.len_utf8())
        };
        self.pos = end;
        Some(Ok(token))
    }
}

impl<'a> Token<'a> {
    // Matching on `*self` copies out the `&'a str` payload, so the result isn't tied to `&self`.
    pub fn lexeme(&self) -> Option<&'a str> {
        match *self {
            Token::Ident(s) | Token::Number(s) | Token::Str(s) => Some(s),
            Token::Punct(_) => None,
        }
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token<'_>>, LexError> {
    Lexer::new(src).collect()
}
