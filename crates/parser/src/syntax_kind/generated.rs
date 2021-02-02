//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. Ident, `UseKw`, or `Struct`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    Tombstone,
    #[doc(hidden)]
    Eof,
    EqEq,
    NotEq,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Eq,
    Walrus,
    Comma,
    Dot,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    ShiftLeft,
    ShiftRight,
    Tilde,
    BitAnd,
    BitOr,
    Lt,
    Gt,
    LtEq,
    GtEq,
    LogicalAnd,
    LogicalOr,
    Bang,
    Unknown,
    FnKw,
    LetKw,
    TrueKw,
    FalseKw,
    IfKw,
    ElseKw,
    ReturnKw,
    NumberLit,
    StringLit,
    Error,
    Ident,
    Whitespace,
    Comment,
    Newline,
    LetStmt,
    IfStmt,
    ReturnStmt,
    CallExpr,
    PrefixExpr,
    BinExpr,
    #[doc(hidden)]
    __LAST,
}
#[doc = r" A helper macro to get the SyntaxKind"]
#[macro_export]
macro_rules ! T { [==] => { $ crate :: SyntaxKind :: EqEq } ; [!=] => { $ crate :: SyntaxKind :: NotEq } ; ['('] => { $ crate :: SyntaxKind :: LParen } ; [')'] => { $ crate :: SyntaxKind :: RParen } ; ['{'] => { $ crate :: SyntaxKind :: LBrace } ; ['}'] => { $ crate :: SyntaxKind :: RBrace } ; ['['] => { $ crate :: SyntaxKind :: LBracket } ; [']'] => { $ crate :: SyntaxKind :: RBracket } ; [:] => { $ crate :: SyntaxKind :: Colon } ; [=] => { $ crate :: SyntaxKind :: Eq } ; [:=] => { $ crate :: SyntaxKind :: Walrus } ; [,] => { $ crate :: SyntaxKind :: Comma } ; [.] => { $ crate :: SyntaxKind :: Dot } ; [;] => { $ crate :: SyntaxKind :: Semicolon } ; [+] => { $ crate :: SyntaxKind :: Plus } ; [-] => { $ crate :: SyntaxKind :: Minus } ; [*] => { $ crate :: SyntaxKind :: Asterisk } ; [/] => { $ crate :: SyntaxKind :: Slash } ; [%] => { $ crate :: SyntaxKind :: Modulo } ; [<<] => { $ crate :: SyntaxKind :: ShiftLeft } ; [>>] => { $ crate :: SyntaxKind :: ShiftRight } ; [~] => { $ crate :: SyntaxKind :: Tilde } ; [&] => { $ crate :: SyntaxKind :: BitAnd } ; [|] => { $ crate :: SyntaxKind :: BitOr } ; [<] => { $ crate :: SyntaxKind :: Lt } ; [>] => { $ crate :: SyntaxKind :: Gt } ; [<=] => { $ crate :: SyntaxKind :: LtEq } ; [>=] => { $ crate :: SyntaxKind :: GtEq } ; [&&] => { $ crate :: SyntaxKind :: LogicalAnd } ; [||] => { $ crate :: SyntaxKind :: LogicalOr } ; [!] => { $ crate :: SyntaxKind :: Bang } ; [unknown] => { $ crate :: SyntaxKind :: Unknown } ; [fn] => { $ crate :: SyntaxKind :: FnKw } ; [let] => { $ crate :: SyntaxKind :: LetKw } ; [true] => { $ crate :: SyntaxKind :: TrueKw } ; [false] => { $ crate :: SyntaxKind :: FalseKw } ; [if] => { $ crate :: SyntaxKind :: IfKw } ; [else] => { $ crate :: SyntaxKind :: ElseKw } ; [return] => { $ crate :: SyntaxKind :: ReturnKw } ; [Number] => { $ crate :: SyntaxKind :: NumberLit } ; [String] => { $ crate :: SyntaxKind :: StringLit } ; [ident] => { $ crate :: SyntaxKind :: Ident } ; [__] => { $ crate :: SyntaxKind :: Tombstone } ; [eof] => { $ crate :: SyntaxKind :: Eof } ; }
