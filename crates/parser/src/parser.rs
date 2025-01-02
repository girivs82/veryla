use crate::parser_error::ParserError;
use crate::resource_table;
use crate::veryla_grammar::VerylaGrammar;
use crate::veryla_grammar_trait::Veryla;
use crate::veryla_parser::parse;
use std::path::Path;

#[derive(Debug)]
pub struct Parser {
    pub veryla: Veryla,
}

impl Parser {
    #[allow(clippy::result_large_err)]
    pub fn parse<T: AsRef<Path>>(input: &str, file: &T) -> Result<Self, ParserError> {
        // Inserting PathId because it will not be inserted if input doesn't have token.
        let _ = resource_table::insert_path(file.as_ref());

        let mut grammar = VerylaGrammar::new();
        parse(input, file, &mut grammar)?;

        let veryla = grammar.veryla.unwrap();

        Ok(Parser { veryla })
    }
}
