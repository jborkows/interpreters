use super::{
    base::{ColumnNumber, LineNumber},
    tokens::{Token, TokenKind},
};

#[derive(Debug, PartialEq, Eq)]
pub struct SourceCharecter {
    ch: char,
    pub column_number: ColumnNumber,
    pub line_number: LineNumber,
}

impl SourceCharecter {
    pub fn new(ch: char, column_number: ColumnNumber, line_number: LineNumber) -> Self {
        Self {
            ch,
            column_number,
            line_number,
        }
    }
}

pub enum ReadingStatus {
    Read(Vec<Token>),
    Finished,
}

pub fn read<T, F>(source: T, output: F)
where
    T: Iterator<Item = SourceCharecter>,
    F: Fn(ReadingStatus) -> (),
{
    let mut state = State::Idle;
    for charecter in source {
        let (newState, tokens) = next(charecter, state);
        state = newState;
        output(ReadingStatus::Read(tokens));
    }
    output(ReadingStatus::Finished);
}

#[derive(Debug, PartialEq, Eq)]
struct StateLineContext {
    text: String,
    line: LineNumber,
    column: ColumnNumber,
}

impl StateLineContext {
    fn as_reading_operator(self) -> State {
        State::ReadingOperator(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Idle,
    ReadingText(StateLineContext),
    ReadingNumber(StateLineContext),
    ReadingOperator(StateLineContext),
}

fn next(charecter: SourceCharecter, state: State) -> (State, Vec<Token>) {
    println!("current: {:?} -> incoming {:?}", state, charecter);
    match state {
        State::Idle => read_idle(&charecter),
        State::ReadingText(context) => read_text(charecter, context),
        State::ReadingNumber(context) => read_number(charecter, context),
        State::ReadingOperator(context) => read_operator(charecter, context),
    }
}

impl StateLineContext {
    fn full_token(&self, token_kind: TokenKind) -> Token {
        Token::full(self.line, self.column, token_kind)
    }
    fn read_text_as_operator(&self) -> TokenKind {
        operator_token_from_text(self.text.as_str())
    }
    fn read_text_as_number(&self) -> TokenKind {
        TokenKind::Integer(self.text.parse().unwrap())
    }
}

fn read_operator(charecter: SourceCharecter, state: StateLineContext) -> (State, Vec<Token>) {
    if is_whitespace(&charecter) || charecter.ch == '\0' {
        return (
            State::Idle,
            vec![state.full_token(state.read_text_as_operator())],
        );
    }
    let operator = operator(&charecter);
    if operator.is_some() {
        return (
            State::ReadingOperator(StateLineContext {
                text: String::from(charecter.ch),
                line: charecter.line_number,
                column: charecter.column_number,
            }),
            vec![state.full_token(state.read_text_as_operator())],
        );
    }
    let sign = sign(&charecter);
    if sign.is_some() {
        return (
            State::Idle,
            vec![
                state.full_token(state.read_text_as_operator()),
                Token::new(&charecter, sign.unwrap()),
            ],
        );
    }
    if charecter.ch.is_ascii_alphabetic() {
        return (
            charecter.as_reading_text(),
            vec![Token::new(&charecter, state.read_text_as_operator())],
        );
    }
    if charecter.ch.is_ascii_digit() {
        return (
            charecter.as_reading_number(),
            vec![Token::new(&charecter, state.read_text_as_operator())],
        );
    }
    todo!()
}

fn operator_token_from_text(text: &str) -> TokenKind {
    let token = match text {
        "=" => TokenKind::Assign(),
        "+" => TokenKind::Plus(),
        _ => TokenKind::Illegal(format!("Unknown operator: {}", text)),
    };
    token
}

fn read_text(charecter: SourceCharecter, context: StateLineContext) -> (State, Vec<Token>) {
    if is_whitespace(&charecter) || charecter.ch == '\0' {
        return (
            State::Idle,
            vec![context.full_token(IdentifierString(&context.text).as_kind())],
        );
    }

    let operator = operator(&charecter);
    if operator.is_some() {
        return (
            charecter.as_reading_operator(),
            vec![context.full_token(IdentifierString(&context.text).as_kind())],
        );
    }

    let sign = sign(&charecter);
    if sign.is_some() {
        return (
            State::Idle,
            vec![
                context.full_token(IdentifierString(&context.text).as_kind()),
                Token::new(&charecter, sign.unwrap()),
            ],
        );
    }
    return (
        State::ReadingText(StateLineContext {
            text: context.text + &charecter.ch.to_string(),
            ..context
        }),
        vec![],
    );
}

struct IdentifierString<'a>(&'a String);
impl IdentifierString<'_> {
    fn as_kind(&self) -> TokenKind {
        match self.0.as_str() {
            "let" => TokenKind::Let(),
            "fn" => TokenKind::Function(),
            // "true" => TokenKind::True(),
            // "false" => TokenKind::False(),
            // "if" => TokenKind::If(),
            // "else" => TokenKind::Else(),
            // "return" => TokenKind::Return(),
            _ => TokenKind::Identifier(self.0.clone()),
        }
    }
}

fn read_number(charecter: SourceCharecter, context: StateLineContext) -> (State, Vec<Token>) {
    if is_whitespace(&charecter) || charecter.ch == '\0' {
        return (
            State::Idle,
            vec![context.full_token(context.read_text_as_number())],
        );
    }
    let operator = operator(&charecter);
    if operator.is_some() {
        return (
            charecter.as_reading_operator(),
            vec![context.full_token(context.read_text_as_number())],
        );
    }

    let sign = sign(&charecter);
    if sign.is_some() {
        return (
            State::Idle,
            vec![
                context.full_token(context.read_text_as_number()),
                Token::new(&charecter, sign.unwrap()),
            ],
        );
    }
    if charecter.ch.is_ascii_digit() {
        return (
            State::ReadingNumber(StateLineContext {
                text: context.text + &charecter.ch.to_string(),
                ..context
            }),
            vec![],
        );
    }
    return (charecter.as_reading_text(), vec![]);
}

fn is_whitespace(charecter: &SourceCharecter) -> bool {
    charecter.ch.is_ascii_whitespace()
}

fn operator(charecter: &SourceCharecter) -> Option<TokenKind> {
    match charecter.ch {
        '=' => Some(TokenKind::Assign()),
        '+' => Some(TokenKind::Plus()),
        _ => None,
    }
}
fn sign(charecter: &SourceCharecter) -> Option<TokenKind> {
    match charecter.ch {
        ',' => Some(TokenKind::Comma()),
        ';' => Some(TokenKind::Semicolon()),
        '(' => Some(TokenKind::LeftParen()),
        ')' => Some(TokenKind::RightParen()),
        '{' => Some(TokenKind::LeftBrace()),
        '}' => Some(TokenKind::RightBrace()),
        _ => None,
    }
}

fn read_idle(charecter: &SourceCharecter) -> (State, Vec<Token>) {
    if is_whitespace(&charecter) || charecter.ch == '\0' {
        return (State::Idle, vec![]);
    }
    let operator = operator(&charecter);
    if operator.is_some() {
        return (charecter.as_reading_operator(), vec![]);
    }
    let sign = sign(&charecter);
    if sign.is_some() {
        return (State::Idle, vec![Token::new(charecter, sign.unwrap())]);
    }
    if charecter.ch.is_ascii_alphabetic() {
        return (charecter.as_reading_text(), vec![]);
    }
    if charecter.ch.is_ascii_digit() {
        return (charecter.as_reading_number(), vec![]);
    }

    todo!()
}

impl SourceCharecter {
    fn as_reading_operator(&self) -> State {
        let charecter = self;
        State::ReadingOperator(self.as_state_line_context())
    }
    fn as_reading_text(&self) -> State {
        let charecter = self;
        State::ReadingText(self.as_state_line_context())
    }
    fn as_reading_number(&self) -> State {
        let charecter = self;
        State::ReadingNumber(self.as_state_line_context())
    }
    fn as_state_line_context(&self) -> StateLineContext {
        let charecter = self;
        StateLineContext {
            text: String::from(charecter.ch),
            line: charecter.line_number,
            column: charecter.column_number,
        }
    }
}
