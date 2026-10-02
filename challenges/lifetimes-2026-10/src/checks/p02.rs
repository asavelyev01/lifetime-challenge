//! P02 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p02_lexer::*;
/// let src = String::from("a + b");
/// let tokens: Vec<Token> = Lexer::new(&src).map(Result::unwrap).collect();
/// assert_eq!(tokens, [Token::Ident("a"), Token::Punct('+'), Token::Ident("b")]);
/// ```
pub fn tokens_outlive_the_lexer() {}

/// ```
/// use lifetimes_2026_10::p02_lexer::*;
/// let src = String::from("first second third");
/// let rest = {
///     let mut lexer = Lexer::new(&src);
///     lexer.next();
///     lexer.remainder() // the lexer dies here
/// };
/// assert_eq!(rest, " second third");
/// ```
pub fn remainder_outlives_the_lexer() {}

/// ```
/// use lifetimes_2026_10::p02_lexer::*;
/// let src = String::from("x = 42 + \"s\"");
/// // `tokenize(..)` is a temporary Vec dropped at the end of this statement:
/// let lexemes: Vec<&str> = tokenize(&src).unwrap().iter().filter_map(Token::lexeme).collect();
/// assert_eq!(lexemes, ["x", "42", "s"]);
/// ```
pub fn lexemes_outlive_the_tokens() {}

/// ```compile_fail
/// use lifetimes_2026_10::p02_lexer::*;
/// let tokens;
/// {
///     let src = String::from("a b");
///     tokens = tokenize(&src).unwrap();
/// }
/// println!("{tokens:?}");
/// ```
pub fn tokens_cannot_outlive_the_source() {}

/// ```compile_fail
/// use lifetimes_2026_10::p02_lexer::*;
/// let rest;
/// {
///     let src = String::from("a b");
///     let mut lexer = Lexer::new(&src);
///     lexer.next();
///     rest = lexer.remainder();
/// }
/// println!("{rest}");
/// ```
pub fn remainder_cannot_outlive_the_source() {}
