#[macro_use]
mod syntax_kind;
mod lexer;
mod grammar;

pub use syntax_kind::SyntaxKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(T![==], SyntaxKind::EqEq);
        assert_eq!(T![let], SyntaxKind::LetKw);
        assert_eq!(T![fn], SyntaxKind::FnKw);
    }
}
