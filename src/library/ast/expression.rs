use crate::tokens::Token;

use super::base::Node;

pub(crate) trait Expression: Node {}

pub(crate) struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.short()
    }
}
impl Expression for Identifier {}
