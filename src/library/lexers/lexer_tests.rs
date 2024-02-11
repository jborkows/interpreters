use super::{
    base::{ColumnNumber, LineNumber, SourceCharecter},
    lexer::read_all,
    tokens::{Token, TokenKind},
};

struct Lines {
    lines: Vec<String>,
    current_line: LineNumber,
    current_column: ColumnNumber,
}

impl Lines {
    fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            current_line: LineNumber(0),
            current_column: ColumnNumber(0),
        }
    }
}

impl Iterator for Lines {
    type Item = SourceCharecter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_line.0 >= self.lines.len() as u16 {
            return None;
        }
        let line = &self.lines[self.current_line.0 as usize];
        if self.current_column.0 >= line.len() as u16 {
            self.current_line.0 += 1;
            self.current_column.0 = 0;
            return self.next();
        }
        let ch = line.chars().nth((self.current_column.0) as usize).unwrap();
        let charecter = SourceCharecter::new(ch, self.current_column + 1, self.current_line + 1);
        self.current_column.0 += 1;
        Some(charecter)
    }
}

#[test]
fn next_sign() {
    let input = Lines::new(vec![String::from("=+(){},;*<;>/;")]);
    let expected = vec![
        (LineNumber(1), ColumnNumber(1), TokenKind::Assign()),
        (LineNumber(1), ColumnNumber(2), TokenKind::Plus()),
        (LineNumber(1), ColumnNumber(3), TokenKind::LeftParen()),
        (LineNumber(1), ColumnNumber(4), TokenKind::RightParen()),
        (LineNumber(1), ColumnNumber(5), TokenKind::LeftBrace()),
        (LineNumber(1), ColumnNumber(6), TokenKind::RightBrace()),
        (LineNumber(1), ColumnNumber(7), TokenKind::Comma()),
        (LineNumber(1), ColumnNumber(8), TokenKind::Semicolon()),
        (LineNumber(1), ColumnNumber(9), TokenKind::Asterisk()),
        (LineNumber(1), ColumnNumber(10), TokenKind::LessThen()),
        (LineNumber(1), ColumnNumber(11), TokenKind::Semicolon()),
        (LineNumber(1), ColumnNumber(12), TokenKind::GreaterThen()),
        (LineNumber(1), ColumnNumber(13), TokenKind::Slash()),
        (LineNumber(1), ColumnNumber(14), TokenKind::Semicolon()),
    ];

    perform_test(input, expected);
}

#[test]
fn euqality_negation() {
    let input = Lines::new(vec![String::from("==!=!;")]);
    let expected = vec![
        (LineNumber(1), ColumnNumber(1), TokenKind::Equality()),
        (LineNumber(1), ColumnNumber(3), TokenKind::Inequality()),
        (LineNumber(1), ColumnNumber(5), TokenKind::Negation()),
        (LineNumber(1), ColumnNumber(6), TokenKind::Semicolon()),
    ];

    perform_test(input, expected);
}

#[test]
fn true_false() {
    let input = Lines::new(vec![String::from("true false trues falses")]);
    let expected = vec![
        (LineNumber(1), ColumnNumber(1), TokenKind::True()),
        (LineNumber(1), ColumnNumber(6), TokenKind::False()),
        (
            LineNumber(1),
            ColumnNumber(12),
            TokenKind::Identifier(String::from("trues")),
        ),
        (
            LineNumber(1),
            ColumnNumber(18),
            TokenKind::Identifier(String::from("falses")),
        ),
    ];

    perform_test(input, expected);
}

#[test]
fn if_else_return() {
    let input = Lines::new(vec![String::from("if ifs else elses return returns")]);
    let expected = vec![
        (LineNumber(1), ColumnNumber(1), TokenKind::If()),
        (
            LineNumber(1),
            ColumnNumber(4),
            TokenKind::Identifier(String::from("ifs")),
        ),
        (LineNumber(1), ColumnNumber(8), TokenKind::Else()),
        (
            LineNumber(1),
            ColumnNumber(13),
            TokenKind::Identifier(String::from("elses")),
        ),
        (LineNumber(1), ColumnNumber(19), TokenKind::Return()),
        (
            LineNumber(1),
            ColumnNumber(26),
            TokenKind::Identifier(String::from("returns")),
        ),
    ];

    perform_test(input, expected);
}

fn perform_test(input: Lines, expected: Vec<(LineNumber, ColumnNumber, TokenKind)>) {
    read_all(input)
        .zip(expected.iter())
        .for_each(|(token, expected)| {
            assert_eq!(token, Token(expected.0, expected.1, expected.2.clone()));
        });
}

#[test]
fn more_complex_text() {
    let input = Lines::new(vec![
        String::from("let five = 5;"),
        String::from("let ten = 10;"),
        String::from(""),
        String::from("let add = fn(x, y) {"),
        String::from("  x + y;"),
        String::from("};"),
        String::from(""),
        String::from("let result = add(five, ten);"),
    ]);
    let expected = vec![
        (LineNumber(1), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(1),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("five")),
        ),
        (LineNumber(1), ColumnNumber(10), TokenKind::Assign()),
        (LineNumber(1), ColumnNumber(12), TokenKind::Integer(5)),
        (LineNumber(1), ColumnNumber(13), TokenKind::Semicolon()),
        (LineNumber(2), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(2),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("ten")),
        ),
        (LineNumber(2), ColumnNumber(9), TokenKind::Assign()),
        (LineNumber(2), ColumnNumber(11), TokenKind::Integer(10)),
        (LineNumber(2), ColumnNumber(13), TokenKind::Semicolon()),
        (LineNumber(4), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(4),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("add")),
        ),
        (LineNumber(4), ColumnNumber(9), TokenKind::Assign()),
        (LineNumber(4), ColumnNumber(11), TokenKind::Function()),
        (LineNumber(4), ColumnNumber(13), TokenKind::LeftParen()),
        (
            LineNumber(4),
            ColumnNumber(14),
            TokenKind::Identifier(String::from("x")),
        ),
        (LineNumber(4), ColumnNumber(15), TokenKind::Comma()),
        (
            LineNumber(4),
            ColumnNumber(17),
            TokenKind::Identifier(String::from("y")),
        ),
        (LineNumber(4), ColumnNumber(18), TokenKind::RightParen()),
        (LineNumber(4), ColumnNumber(20), TokenKind::LeftBrace()),
        (
            LineNumber(5),
            ColumnNumber(3),
            TokenKind::Identifier(String::from("x")),
        ),
        (LineNumber(5), ColumnNumber(5), TokenKind::Plus()),
        (
            LineNumber(5),
            ColumnNumber(7),
            TokenKind::Identifier(String::from("y")),
        ),
        (LineNumber(5), ColumnNumber(8), TokenKind::Semicolon()),
        (LineNumber(6), ColumnNumber(1), TokenKind::RightBrace()),
        (LineNumber(6), ColumnNumber(2), TokenKind::Semicolon()),
        (LineNumber(8), ColumnNumber(1), TokenKind::Let()),
        (
            LineNumber(8),
            ColumnNumber(5),
            TokenKind::Identifier(String::from("result")),
        ),
        (LineNumber(8), ColumnNumber(12), TokenKind::Assign()),
        (
            LineNumber(8),
            ColumnNumber(14),
            TokenKind::Identifier(String::from("add")),
        ),
        (LineNumber(8), ColumnNumber(17), TokenKind::LeftParen()),
        (
            LineNumber(8),
            ColumnNumber(18),
            TokenKind::Identifier(String::from("five")),
        ),
        (LineNumber(8), ColumnNumber(22), TokenKind::Comma()),
        (
            LineNumber(8),
            ColumnNumber(24),
            TokenKind::Identifier(String::from("ten")),
        ),
        (LineNumber(8), ColumnNumber(27), TokenKind::RightParen()),
        (LineNumber(8), ColumnNumber(28), TokenKind::Semicolon()),
    ];

    perform_test(input, expected);
}
