use crate::lexers::{
    read_idle::read_idle, read_number::read_number, read_operator::read_operator,
    read_text::read_text,
};

use super::{
    base::{SourceCharecter, State},
    tokens::Token,
};

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
        let (new_state, tokens) = next(charecter, state);
        state = new_state;
        output(ReadingStatus::Read(tokens));
    }
    output(ReadingStatus::Finished);
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
