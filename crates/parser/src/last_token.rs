use crate::veryla_token::{Token, VerylaToken};
use crate::veryla_walker::VerylaWalker;

#[derive(Default)]
pub struct LastToken {
    token: Option<Token>,
}

impl LastToken {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn token(&self) -> &Option<Token> {
        &self.token
    }
}

impl VerylaWalker for LastToken {
    /// Semantic action for non-terminal 'VerylaToken'
    fn veryla_token(&mut self, arg: &VerylaToken) {
        self.token = Some(arg.token);
    }
}
