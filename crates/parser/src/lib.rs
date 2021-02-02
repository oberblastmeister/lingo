#[macro_use]
mod syntax_kind;
mod grammar;
mod lexer;

pub use syntax_kind::SyntaxKind;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    fn multi_assert<T>(assertions: &[(T, T)])
    where
        T: fmt::Debug + PartialEq,
    {
        for assertion in assertions {
            assert_eq!(assertion.0, assertion.1);
        }
    }

    #[test]
    fn it_works() {
        multi_assert(&[
            (T![==], SyntaxKind::EqEq),
            (T![ident], SyntaxKind::Ident),
            (T![fn], SyntaxKind::FnKw),
            (T![if], SyntaxKind::IfKw),
            (T![')'], SyntaxKind::RParen),
        ])
    }
}
