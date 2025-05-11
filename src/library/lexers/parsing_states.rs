use crate::lines::TextPosition;

pub(super) enum LexerState {
    Idle,
    ReadingEquality { starting_position: TextPosition },
    ReadingNegation { starting_position: TextPosition },
}
