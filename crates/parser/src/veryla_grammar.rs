use crate::veryla_grammar_trait::*;
use parol_runtime::ParolError;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, Default)]
pub struct VerylaGrammar {
    pub veryla: Option<Veryla>,
}

impl VerylaGrammar {
    pub fn new() -> Self {
        VerylaGrammar::default()
    }
}

impl Display for Veryla {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{self:?}")
    }
}

impl Display for VerylaGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.veryla {
            Some(veryla) => writeln!(f, "{veryla}"),
            None => write!(f, "No parse result"),
        }
    }
}

impl VerylaGrammarTrait for VerylaGrammar {
    /// Semantic action for non-terminal 'Veryla'
    fn veryla(&mut self, arg: &Veryla) -> Result<(), ParolError> {
        self.veryla = Some(arg.clone());
        Ok(())
    }
}
