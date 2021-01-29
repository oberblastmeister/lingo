//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    Tombstone,
    #[doc(hidden)]
    Eof,
    Eq,
    EqEq,
    NotEq,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Semicolon,
    Bang,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lt,
    Gt,
    LtEq,
    GtEq,
    FnKw,
    LetKw,
    TrueKw,
    FalseKw,
    IfKw,
    ElseKw,
    ReturnKw,
    NumberLit,
    StringLit,
    #[doc(hidden)]
    __LAST,
}
#[macro_export]
macro_rules ! T { [=] => { $ crate :: SyntaxKind :: Eq } ; [==] => { $ crate :: SyntaxKind :: EqEq } ; [!=] => { $ crate :: SyntaxKind :: NotEq } ; ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; ['{'] => { $ crate :: SyntaxKind :: LBrace } ; ['}'] => { $ crate :: SyntaxKind :: RBrace } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; [,] => { $ crate :: SyntaxKind :: Comma } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [!] => { $ crate :: SyntaxKind :: Bang } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [*] => { $ crate :: SyntaxKind :: Asterisk } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [<] => { $ crate :: SyntaxKind :: Lt } ; [>] => { $ crate :: SyntaxKind :: Gt } ; [<=] => { $ crate :: SyntaxKind :: LtEq } ; [>=] => { $ crate :: SyntaxKind :: GtEq } ; [fn] => { $ crate :: SyntaxKind :: FnKw } ; [let] => { $ crate :: SyntaxKind :: LetKw } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [if] => { $ crate :: SyntaxKind :: IfKw } ; [else] => { $ crate :: SyntaxKind :: ElseKw } ; [return] => { $ crate :: SyntaxKind :: ReturnKw } ; [Number] => { $ crate :: SyntaxKind :: NumberLit } ; [String] => { $ crate :: SyntaxKind :: StringLit } ; }
