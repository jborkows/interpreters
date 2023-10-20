use super::base::{ColumnNumber, Lexable, LineNumber};

#[derive(Debug, PartialEq, Eq)]
pub struct Token(pub LineNumber, pub ColumnNumber, pub TokenKind);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Illegal(),
    EOF(),
    Identifier(String),
    Integer(i32),

    Assign(),
    Plus(),

    Comma(),
    Semicolon(),
    LeftParen(),
    RightParen(),
    LeftBrace(),
    RightBrace(),

    Function(),
    Let(),
}

struct Lexer {
    input: String,
    line_number: LineNumber,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String, line_number: LineNumber) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            line_number,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let column_number = ColumnNumber((self.position + 1) as u16);
        let tok = match self.ch {
            '=' => Token(self.line_number, column_number, TokenKind::Assign()),
            '+' => Token(self.line_number, column_number, TokenKind::Plus()),
            ',' => Token(self.line_number, column_number, TokenKind::Comma()),
            ';' => Token(self.line_number, column_number, TokenKind::Semicolon()),
            '(' => Token(self.line_number, column_number, TokenKind::LeftParen()),
            ')' => Token(self.line_number, column_number, TokenKind::RightParen()),
            '{' => Token(self.line_number, column_number, TokenKind::LeftBrace()),
            '}' => Token(self.line_number, column_number, TokenKind::RightBrace()),
            _ => Token(self.line_number, column_number, TokenKind::EOF()),
        };
        self.read_char();
        tok
    }
}

pub struct NextToken<'a, T>
where
    T: Lexable,
{
    lexable: &'a T,
    current_line_number: LineNumber,
    lexer: Option<Lexer>,
    finished: bool,
}

impl<'a, T> NextToken<'a, T>
where
    T: Lexable,
{
    pub fn new(lexable: &'a T) -> Self {
        Self {
            lexable,
            current_line_number: LineNumber(0),
            lexer: None,
            finished: false,
        }
    }
}

impl<'a, T> Iterator for NextToken<'a, T>
where
    T: Lexable,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match self.lexer.as_mut() {
            Some(lexer) => {
                let tok = lexer.next_token();
                if tok.2 == TokenKind::EOF() {
                    self.lexer = None;
                    self.finished = true;
                }
                Some(tok)
            }
            None => match self.lexable.next_line() {
                Some((line_number, text)) => {
                    self.current_line_number = line_number;
                    self.lexer = Some(Lexer::new(text, line_number));
                    self.next()
                }
                None => {
                    self.finished = true;
                    Some(Token(
                        LineNumber(self.current_line_number.0),
                        ColumnNumber(1),
                        TokenKind::EOF(),
                    ))
                }
            },
        }
    }
}
