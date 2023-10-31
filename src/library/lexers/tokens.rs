use super::{
    base::{ColumnNumber, Lexable, LineNumber},
    lexer::SourceCharecter,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Token(pub LineNumber, pub ColumnNumber, pub TokenKind);

impl Token {
    pub fn new(source_charecter: &SourceCharecter, token_kind: TokenKind) -> Self {
        Self(
            source_charecter.line_number,
            source_charecter.column_number,
            token_kind,
        )
    }
    pub fn full(
        line_number: LineNumber,
        column_number: ColumnNumber,
        token_kind: TokenKind,
    ) -> Self {
        Self(line_number, column_number, token_kind)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Illegal(String),
    EOF(),
    EndOfLine(),
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
        self.skip_whitespace();
        println!("ch: {}", self.ch);
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
            '\0' => Token(self.line_number, column_number, TokenKind::EndOfLine()),
            _ => {
                // TODO fix moving cursor with read_char
                if is_letter(self.ch) {
                    self.extract_identifier(column_number)
                } else if is_number(self.ch) {
                    self.extract_number(column_number)
                } else {
                    Token(
                        self.line_number,
                        column_number,
                        TokenKind::Illegal(String::from(&self.ch.to_string())),
                    )
                }
            }
        };
        self.read_char();
        tok
    }

    fn extract_number(&mut self, column_number: ColumnNumber) -> Token {
        let mut number = String::new();
        while is_number(self.ch) {
            number.push(self.ch);
            self.read_char();
        }
        Token(
            self.line_number,
            column_number,
            TokenKind::Integer(number.parse().unwrap()),
        )
    }

    fn extract_identifier(&mut self, column_number: ColumnNumber) -> Token {
        let mut ident = String::new();
        while is_letter(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }
        Token(
            self.line_number,
            column_number,
            token_kind_based_on_string(&ident),
        )
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
fn token_kind_based_on_string(s: &str) -> TokenKind {
    match s {
        "fn" => TokenKind::Function(),
        "let" => TokenKind::Let(),
        _ => TokenKind::Identifier(String::from(s)),
    }
}
fn is_number(ch: char) -> bool {
    ch.is_ascii_digit()
}
fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
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
                    Some(tok)
                } else if tok.2 == TokenKind::EndOfLine() {
                    self.lexer = None;
                    self.next()
                } else {
                    Some(tok)
                }
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
                        LineNumber(self.current_line_number.0 + 1),
                        ColumnNumber(1),
                        TokenKind::EOF(),
                    ))
                }
            },
        }
    }
}
