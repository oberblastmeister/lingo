//! Based on rustc_lexer with inspiration from the Rob Pike [lexer](https://www.youtube.com/watch?v=HxaD_trXwRE)

use std::convert::TryFrom;
use std::str::Chars;

use crate::SyntaxKind;
use rowan::TextSize;

/// A lingo token
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The length of the token.
    pub len: TextSize,
}

impl Token {
    fn new(kind: SyntaxKind, len: u32) -> Token {
        Token {
            kind,
            len: TextSize::from(len),
        }
    }
}

pub struct Lexer<'a> {
    input_len: u32,
    chars: Chars<'a>,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input_len: u32::try_from(input.len()).unwrap(),
            chars: input.chars(),
        }
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    fn peek_nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }

    /// Peeks next char from stream without consuming it
    fn peek_char(&self) -> char {
        self.peek_nth_char(0)
    }

    fn bump_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn chars_len(&self) -> u32 {
        u32::try_from(self.chars.as_str().len()).unwrap()
    }

    /// Checks if there is nothing more to consume.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Returns amount of already consumed symbols.
    pub(crate) fn len_consumed(&self) -> u32 {
        self.input_len - self.chars_len()
    }

    fn next_token(&mut self) -> Token {
        Token::new(self.lex_main(), self.len_consumed())
    }

    fn lex_main(&mut self) -> SyntaxKind {
        let next_char = self.bump_char().unwrap();
        let peek_char = self.peek_char();

        match next_char {
            // multichar tokens
            '/' => match self.peek_char() {
                '/' => self.line_comment(),
                _ => T![/],
            },

            '=' => match peek_char {
                '=' => T![==],
                _ => T![=],
            },

            '!' => match peek_char {
                '=' => T![!=],
                _ => T![!],
            },

            '<' => match peek_char {
                '<' => T![<<],
                '=' => T![<=],
                _ => T![<],
            },

            '>' => match peek_char {
                '>' => T![>>],
                '=' => T![>=],
                _ => T![>],
            },

            ':' => match peek_char {
                '=' => T![:=],
                _ => T![:],
            },

            ';' => T![;],
            ',' => T![,],
            '.' => T![.],

            '(' => T!['('],
            ')' => T![')'],
            '[' => T!['['],
            ']' => T![']'],
            '{' => T!['{'],
            '}' => T!['}'],

            // operators
            '*' => T![*],
            '%' => T![%],
            '+' => T![+],
            '-' => T![-],
            '~' => T![~],

            // literals
            '"' => self.string_lit(),
            '0'..='9' => self.number_lit(),

            // special
            _ if is_whitespace(next_char) => self.whitespace(),
            _ if is_ident_start(next_char) => self.ident(),

            _ => T![unknown],
        }
    }

    fn line_comment(&mut self) -> SyntaxKind {
        todo!()
    }

    fn whitespace(&mut self) -> SyntaxKind {
        todo!()
    }

    fn string_lit(&mut self) -> SyntaxKind {
        todo!()
    }

    fn number_lit(&mut self) -> SyntaxKind {
        todo!()
    }

    fn ident(&mut self) -> SyntaxKind {
        todo!()
    }
}

pub fn is_ident_start(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..'Z').contains(&c) || c == '_'
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}
