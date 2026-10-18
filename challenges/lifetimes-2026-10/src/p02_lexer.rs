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

use std::{iter::Peekable, str::CharIndices};

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
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src,
            chars: src.char_indices().peekable(),
        }
    }

    /// Byte offset in the source where [`Lexer::remainder`] starts, i.e.
    /// `&src[lexer.offset()..] == lexer.remainder()` always holds.
    ///
    /// `0` initially; right after the end of the last token yielded; `src.len()` once the lexer
    /// is exhausted or has returned an error.
    // TODO(challenge): implement
    pub fn offset(&mut self) -> usize {
        self.chars.peek().map(|(p, _)| *p).unwrap_or(self.src.len())
    }

    /// The not-yet-consumed part of the source: everything after the last token yielded,
    /// **including** any leading whitespace (it is not skipped here).
    ///
    /// Initially the whole source; `""` once the lexer is exhausted (has returned `None`) or has
    /// returned an error. Example: after the first `next()` on `"let x"`, `remainder() == " x"`.
    ///
    /// Lifetimes: the result borrows from the **source** (`'a`), not from the lexer — it must stay
    /// usable after the lexer is dropped.
    // TODO(challenge): implement + fix the signature
    pub fn remainder(&mut self) -> &'a str {
        &self.src[self.offset()..]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError>;

    /// Yields the next token, following the lexing rules in the module docs:
    /// skip whitespace, then `Ident` / `Number` / `Str` / single-char `Punct`.
    ///
    /// An unterminated string literal yields `Err(LexError::UnterminatedString { start })` (byte
    /// offset of the opening quote) exactly once; after that, and after the end of input, returns
    /// `None` forever.
    // TODO(challenge): implement
    fn next(&mut self) -> Option<Self::Item> {
        #[derive(Copy, Clone)]
        enum TokenType {
            Ident,
            Number,
            Str,
            Punct,
        }
        impl TokenType {
            fn from_first(first: char) -> Self {
                match first {
                    c if c == '_' || c.is_alphabetic() => Self::Ident,
                    c if c.is_ascii_digit() => Self::Number,
                    '"' => Self::Str,
                    _ => Self::Punct,
                }
            }

            pub fn ended_by(&self, c: char) -> bool {
                match self {
                    TokenType::Ident => !(c == '_' || c.is_alphanumeric()),
                    TokenType::Number => !c.is_ascii_digit(),
                    TokenType::Str => c == '"',
                    TokenType::Punct => true,
                }
            }
        }
        struct CurrentToken {
            start: usize,
            pub token_type: TokenType,
        }

        let mut current_token: Option<CurrentToken> = None;

        loop {
            match (self.chars.peek().cloned(), &current_token) {
                (None, None) => {
                    self.chars.next();
                    return None;
                }
                (Some((_, c)), None) if c.is_whitespace() => {
                    self.chars.next();
                }
                (Some((pos, c)), None) => {
                    self.chars.next();
                    let token_type = TokenType::from_first(c);

                    match token_type {
                        TokenType::Punct => return Some(Ok(Token::Punct(c))),
                        _ => {
                            current_token = Some(CurrentToken {
                                start: pos,
                                token_type,
                            })
                        }
                    }
                }
                (Some((pos, c)), Some(CurrentToken { start, token_type }))
                    if token_type.ended_by(c) =>
                {
                    return Some(Ok({
                        match &token_type {
                            TokenType::Ident => Token::Ident(&self.src[*start..pos]),
                            TokenType::Number => Token::Number(&self.src[*start..pos]),
                            TokenType::Punct => Token::Punct(c),
                            TokenType::Str => {
                                self.chars.next();
                                Token::Str(&self.src[start + 1..pos])
                            }
                        }
                    }));
                }
                (None, Some(CurrentToken { start, token_type })) => {
                    self.chars.next();
                    return {
                        match &token_type {
                            TokenType::Ident => Some(Ok(Token::Ident(&self.src[*start..]))),
                            TokenType::Number => Some(Ok(Token::Number(&self.src[*start..]))),
                            TokenType::Punct => panic!("Unexpected"),
                            TokenType::Str => {
                                return Some(Err(LexError::UnterminatedString { start: *start }));
                            }
                        }
                    };
                }
                _ => {
                    self.chars.next();
                }
            };
        }
    }
}

impl<'a> Token<'a> {
    /// The source text of the token: the identifier / digits for `Ident` / `Number`, the payload
    /// **without quotes** for `Str`, and `None` for `Punct` (a `char` has no borrowed text).
    ///
    /// Lifetimes: the result borrows from the **source** (`'a`), not from the token — it must stay
    /// usable after the token (or the `Vec` holding it) is dropped.
    // TODO(challenge): implement + fix the signature
    pub fn lexeme(&self) -> Option<&'a str> {
        match self {
            Token::Ident(s) | Token::Number(s) | Token::Str(s) => Some(s),
            Token::Punct(_) => None,
        }
    }
}

/// Lexes the whole source: all tokens in order, or the first [`LexError`].
///
/// Example: `tokenize("a+1") == Ok(vec![Ident("a"), Punct('+'), Number("1")])`.
// TODO(challenge): implement
pub fn tokenize<'a>(src: &'a str) -> Result<Vec<Token<'a>>, LexError> {
    Lexer::new(src).collect()
}
