use crate::lexers::{
    read_idle::read_idle, read_number::read_number, read_operator::read_operator,
    read_text::read_text,
};

use super::{
    base::{SourceCharecter, State},
    tokens::Token,
};

pub fn read_all<T>(source: T) -> impl Iterator<Item = Token>
where
    T: IntoIterator<Item = SourceCharecter>,
{
    let mut state = State::Idle;
    source.into_iter().flat_map(move |character| {
        let (new_state, new_tokens) = next(character, state.clone());
        state = new_state;
        new_tokens.into_iter()
    })
}

//write read_all function but returning Iterator

fn next(charecter: SourceCharecter, state: State) -> (State, Vec<Token>) {
    // println!("current: {:?} -> incoming {:?}", state, charecter);
    match state {
        State::Idle => read_idle(&charecter),
        State::ReadingText(context) => read_text(charecter, context),
        State::ReadingNumber(context) => read_number(charecter, context),
        State::ReadingOperator(context) => read_operator(charecter, context),
    }
}
