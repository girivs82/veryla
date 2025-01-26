use std::cell::RefCell;
use std::fmt;
use veryla_parser::resource_table::{self, StrId};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Unsafe {
    Pdc,
}

impl fmt::Display for Unsafe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Unsafe::Pdc => "pdc".to_string(),
        };
        text.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub enum UnsafeError {
    UnknownUnsafe,
}

struct Pattern {
    pub pdc: StrId,
}

impl Pattern {
    fn new() -> Self {
        Self {
            pdc: resource_table::insert_str("pdc"),
        }
    }
}

thread_local!(static PAT: RefCell<Pattern> = RefCell::new(Pattern::new()));

impl TryFrom<&veryla_parser::veryla_grammar_trait::UnsafeBlock> for Unsafe {
    type Error = UnsafeError;

    fn try_from(
        value: &veryla_parser::veryla_grammar_trait::UnsafeBlock,
    ) -> Result<Self, Self::Error> {
        PAT.with_borrow(|pat| match value.identifier.identifier_token.token.text {
            x if x == pat.pdc => Ok(Unsafe::Pdc),
            _ => Err(UnsafeError::UnknownUnsafe),
        })
    }
}
