use std::mem;
use tydi_lexer::TokenKind;

/// Syntax tokens and nodes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum SyntaxKind {
    // Safety:
    // - The variants in this enum **must** extend TokenKind from lexer.
    Type,
    Streamlet,
    Equals,
    Semicolon,
    Identifier,
    Whitespace,
    Comment,
    Error,
    // ^ tokens
    // ---------------------
    // ˅ nodes
    TypeDefinition,
    StreamletDefinition,
    Statement,

    // Safety:
    // - This **must** be the last variant of this enum.
    // - The transmute on rowan::Language::kind_from_raw depends on it.
    Root,
}

impl From<TokenKind> for SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        // Safety:
        // - SyntaxKind is always an extension of TokenKind.
        unsafe { mem::transmute::<u16, SyntaxKind>(kind as u16) }
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tydi {}

impl rowan::Language for Tydi {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Root as u16);
        // Safety:
        // - Root variant is always the last variant of the SyntaxKind enum.
        unsafe { mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxToken = rowan::SyntaxToken<Tydi>;
pub type SyntaxNode = rowan::SyntaxNode<Tydi>;
pub type SyntaxElement = rowan::SyntaxElement<Tydi>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Tydi>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<Tydi>;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! kind_eq {
        ($kind:ident) => {
            assert_eq!(SyntaxKind::$kind, SyntaxKind::from(TokenKind::$kind));
        };
    }

    #[test]
    fn token_kind() {
        kind_eq!(Type);
        kind_eq!(Streamlet);
        kind_eq!(Equals);
        kind_eq!(Semicolon);
        kind_eq!(Identifier);
        kind_eq!(Whitespace);
        kind_eq!(Comment);
        kind_eq!(Error);
    }
}
