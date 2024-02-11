use super::{
    base::{SourceCharecter, State, StateLineContext},
    processors::{operator, sign},
    tokens::{Token, TokenKind},
};

pub(crate) fn read_text(
    charecter: SourceCharecter,
    context: StateLineContext,
) -> (State, Vec<Token>) {
    if charecter.is_whitespace() || charecter.ch == '\0' {
        return (
            State::Idle,
            vec![context.full_token(classify_identifier(&context.text))],
        );
    }

    let operator = operator(&charecter);
    if operator.is_some() {
        return (
            charecter.as_reading_operator(),
            vec![context.full_token(classify_identifier(&context.text))],
        );
    }

    let sign = sign(&charecter);
    if sign.is_some() {
        return (
            State::Idle,
            vec![
                context.full_token(classify_identifier(&context.text)),
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

fn classify_identifier(identifier: &String) -> TokenKind {
    match identifier.as_str() {
        "let" => TokenKind::Let(),
        "fn" => TokenKind::Function(),
        "true" => TokenKind::True(),
        "false" => TokenKind::False(),
        _ => TokenKind::Identifier(identifier.clone()),
    }
}
