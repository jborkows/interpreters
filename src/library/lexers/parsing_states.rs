use crate::lines::TextPosition;

#[derive(Debug)]
pub(super) enum LexerState {
    Idle,
    ReadingEquality {
        starting_position: TextPosition,
    },
    ReadingNegation {
        starting_position: TextPosition,
    },
    ReadingMinus {
        starting_position: TextPosition,
    },
    ReadingNumber {
        starting_position: TextPosition,
        value: i32,
        negative: bool,
    },
    ReadingInvalid {
        starting_position: TextPosition,
        reason: String,
    },
}
