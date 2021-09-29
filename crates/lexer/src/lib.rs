//! Lexer for [Tydi](https://github.com/tydi-lang/tydi).
//!
//! # Example
//!
//! ## Iterate over tokens
//!
//! ```rust
//! use tydi_lexer::{Lexer, Token, TokenKind};
//!
//! let input = "type Byte = Bits<8>;";
//! let mut lexer = Lexer::new(input);
//!
//! let Token { kind, text } = lexer.next().unwrap();
//! assert_eq!(kind, TokenKind::Type);
//! assert_eq!(text, "type");
//!
//! let token = lexer.next().unwrap();
//! assert!(token.is_trivia());
//! ```
//!
//! ## Collect all tokens
//!
//! ```rust
//! use tydi_lexer::{Lexer, Token};
//!
//! let input = "type Byte = Bits<8>;";
//! let tokens = Lexer::new(input).collect::<Vec<Token>>();
//!
//! assert_eq!(tokens.len(), 11);
//! ```

use logos::Logos;
use std::{convert::TryInto, ops::Range};
use text_size::TextRange;

/// Tokens, the primitive productions used in the grammar rules.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Logos)]
#[repr(u16)]
pub enum TokenKind {
    // Keywords
    #[token("type")]
    Type,
    #[token("streamlet")]
    Streamlet,

    // Punctuation
    #[token("=")]
    Equals,
    #[token(";")]
    Semicolon,

    // Literals
    #[regex("[a-zA-Z][a-zA-Z_]*")]
    Identifier,

    // Trivia
    #[regex(r"[\p{White_Space}]+")]
    Whitespace,

    #[regex(r"//[^\n]*")]
    Comment,

    #[error]
    Error,
}

impl TokenKind {
    /// Returns [true] when the token is a literal token:
    /// - [TokenKind::Identifier]
    pub fn is_literal(self) -> bool {
        matches!(self, TokenKind::Identifier)
    }

    /// Returns [true] when the token is a trivia token:
    /// - [TokenKind::Comment]
    /// - [TokenKind::Error]
    /// - [TokenKind::Whitespace]
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            TokenKind::Comment | TokenKind::Error | TokenKind::Whitespace
        )
    }
}

/// Lexer produces tokens from a source input.
pub struct Lexer<'src>(logos::Lexer<'src, TokenKind>);

impl<'src> Lexer<'src> {
    /// Returns a new lexer for the given input.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tydi_lexer::Lexer;
    ///
    /// let input = "type Byte = Bits<8>;";
    /// let mut lexer = Lexer::new(input);
    /// ```
    pub fn new(input: &'src str) -> Self {
        Self(TokenKind::lexer(input))
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.0.next()?;
        let text = self.0.slice();
        let Range { start, end } = self.0.span();
        let range = TextRange::new(start.try_into().unwrap(), end.try_into().unwrap());
        Some(Token { kind, text, range })
    }
}

/// Token with kind and source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub text: &'src str,
    pub range: TextRange,
}

impl Token<'_> {
    /// Returns [true] when the token is a literal token.
    ///
    /// See [TokenKind::is_trivia] for more information.
    pub fn is_literal(self) -> bool {
        self.kind.is_literal()
    }

    /// Returns [true] when the token is a trivia token.
    ///
    /// See [TokenKind::is_trivia] for more information.
    pub fn is_trivia(self) -> bool {
        self.kind.is_trivia()
    }
}
