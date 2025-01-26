use crate::analyzer_error::AnalyzerError;
use crate::symbol::{ProtoIncompatible, SymbolKind};
use crate::symbol_table;
use veryla_parser::veryla_grammar_trait::*;
use veryla_parser::veryla_walker::{Handler, HandlerPoint};
use veryla_parser::ParolError;

#[derive(Default)]
pub struct CheckProto<'a> {
    pub errors: Vec<AnalyzerError>,
    text: &'a str,
    point: HandlerPoint,
}

impl<'a> CheckProto<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Handler for CheckProto<'_> {
    fn set_point(&mut self, p: HandlerPoint) {
        self.point = p;
    }
}

impl VerylaGrammarTrait for CheckProto<'_> {
    fn entity_declaration(&mut self, arg: &EntityDeclaration) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if let Some(ref x) = arg.entity_declaration_opt1 {
                if let Ok(symbol) = symbol_table::resolve(x.scoped_identifier.as_ref()) {
                    if let SymbolKind::ProtoEntity(proto) = symbol.found.kind {
                        if let Ok(entity) = symbol_table::resolve(arg.identifier.as_ref()) {
                            if let SymbolKind::Entity(entity) = entity.found.kind {
                                let errors = proto.check_compat(&entity);
                                for error in errors {
                                    let cause = match error {
                                        ProtoIncompatible::MissingParam(x) => {
                                            format!("parameter {x} is missing")
                                        }
                                        ProtoIncompatible::MissingPort(x) => {
                                            format!("port {x} is missing")
                                        }
                                        ProtoIncompatible::UnnecessaryParam(x) => {
                                            format!("parameter {x} is unnecessary")
                                        }
                                        ProtoIncompatible::UnnecessaryPort(x) => {
                                            format!("port {x} is unnecessary")
                                        }
                                        ProtoIncompatible::IncompatibleParam(x) => {
                                            format!("parameter {x} has incompatible type")
                                        }
                                        ProtoIncompatible::IncompatiblePort(x) => {
                                            format!("port {x} has incompatible type")
                                        }
                                    };
                                    self.errors.push(AnalyzerError::incompat_proto(
                                        &arg.identifier.identifier_token.to_string(),
                                        &symbol.found.token.to_string(),
                                        &cause,
                                        self.text,
                                        &arg.identifier.identifier_token.token.into(),
                                    ));
                                }
                            }
                        }
                    } else {
                        self.errors.push(AnalyzerError::mismatch_type(
                            &symbol.found.token.to_string(),
                            "entity prototype",
                            &symbol.found.kind.to_kind_name(),
                            self.text,
                            &x.scoped_identifier.identifier().token.into(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
