use crate::analyzer_error::AnalyzerError;
use crate::r#unsafe::Unsafe;
use crate::symbol::{PowerDomain, SymbolId, SymbolKind};
use crate::symbol_table;
use crate::unsafe_table;
use std::collections::HashMap;
use veryla_parser::resource_table::StrId;
use veryla_parser::veryla_grammar_trait::*;
use veryla_parser::veryla_token::{Token, TokenRange};
use veryla_parser::veryla_walker::{Handler, HandlerPoint};
use veryla_parser::ParolError;

#[derive(Default)]
pub struct CheckPowerDomain<'a> {
    pub errors: Vec<AnalyzerError>,
    text: &'a str,
    point: HandlerPoint,
    expr_power_domains: Vec<(PowerDomain, TokenRange)>,
    inst_power_domains: HashMap<StrId, (PowerDomain, TokenRange)>,
    sequence_power_domain: Option<(PowerDomain, TokenRange)>,
    default_power: Option<SymbolId>,
}

impl<'a> CheckPowerDomain<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    fn push_expr_power_domain(&mut self, kind: &SymbolKind, range: TokenRange) {
        match kind {
            SymbolKind::Variable(x) => {
                self.expr_power_domains.push((x.power_domain, range));
            }
            SymbolKind::Port(x) => {
                self.expr_power_domains.push((x.power_domain, range));
            }
            _ => (),
        }
    }

    fn check_expr_power_domains(&mut self, token: &Token) -> PowerDomain {
        let mut prev: Option<(PowerDomain, TokenRange)> = self.sequence_power_domain;
        for curr in &self.expr_power_domains {
            if let Some(prev) = prev {
                if !curr.0.compatible(&prev.0) && !unsafe_table::contains(token, Unsafe::Cdc) {
                    self.errors.push(AnalyzerError::mismatch_power_domain(
                        &curr.0.to_string(),
                        &prev.0.to_string(),
                        self.text,
                        &curr.1,
                        &prev.1,
                    ));
                }
            }

            prev = Some(*curr);
        }
        prev.map(|(x, _)| x).unwrap_or(PowerDomain::None)
    }
}

impl Handler for CheckPowerDomain<'_> {
    fn set_point(&mut self, p: HandlerPoint) {
        self.point = p;
    }
}

impl VerylaGrammarTrait for CheckPowerDomain<'_> {
    fn scoped_identifier(&mut self, arg: &ScopedIdentifier) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if let Ok(symbol) = symbol_table::resolve(arg) {
                self.push_expr_power_domain(&symbol.found.kind, arg.into());
            }
        }
        Ok(())
    }

    fn let_statement(&mut self, arg: &LetStatement) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.expr_power_domains.clear(),
            HandlerPoint::After => {
                if let Ok(symbol) = symbol_table::resolve(arg.identifier.as_ref()) {
                    self.push_expr_power_domain(&symbol.found.kind, arg.identifier.as_ref().into());
                }
                self.check_expr_power_domains(&arg.semicolon.semicolon_token.token);
            }
        }
        Ok(())
    }

    fn identifier_statement(&mut self, arg: &IdentifierStatement) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.expr_power_domains.clear(),
            HandlerPoint::After => {
                // power domain is assigned to base identifier
                let ident = arg.expression_identifier.scoped_identifier.as_ref();
                if let Ok(symbol) = symbol_table::resolve(ident) {
                    self.push_expr_power_domain(
                        &symbol.found.kind,
                        arg.expression_identifier.as_ref().into(),
                    );
                }

                self.check_expr_power_domains(&arg.semicolon.semicolon_token.token);
            }
        }
        Ok(())
    }

    fn let_declaration(&mut self, arg: &LetDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.expr_power_domains.clear(),
            HandlerPoint::After => {
                if let Ok(symbol) = symbol_table::resolve(arg.identifier.as_ref()) {
                    self.push_expr_power_domain(&symbol.found.kind, arg.identifier.as_ref().into());
                }
                self.check_expr_power_domains(&arg.semicolon.semicolon_token.token);
            }
        }
        Ok(())
    }

    fn sequence_declaration(&mut self, arg: &SequenceDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => {
                let range: TokenRange = arg.sequence.sequence_token.token.into();
                if let Some(ref x) = arg.sequence_declaration_opt {
                    // power domain is assigned to base identifier
                    let ident = x
                        .sequence_event_list
                        .sequence_power
                        .hierarchical_identifier
                        .identifier
                        .as_ref();
                    if let Ok(symbol) = symbol_table::resolve(ident) {
                        match symbol.found.kind {
                            SymbolKind::Port(x) => {
                                self.sequence_power_domain = Some((x.power_domain, range))
                            }
                            SymbolKind::Variable(x) => {
                                self.sequence_power_domain = Some((x.power_domain, range))
                            }
                            _ => (),
                        }
                    }
                } else if let Some(ref x) = self.default_power {
                    if let Some(symbol) = symbol_table::get(*x) {
                        match symbol.kind {
                            SymbolKind::Port(x) => {
                                self.sequence_power_domain = Some((x.power_domain, range))
                            }
                            SymbolKind::Variable(x) => {
                                self.sequence_power_domain = Some((x.power_domain, range))
                            }
                            _ => (),
                        }
                    }
                }
            }
            HandlerPoint::After => self.sequence_power_domain = None,
        }
        Ok(())
    }

    fn assign_declaration(&mut self, arg: &AssignDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.expr_power_domains.clear(),
            HandlerPoint::After => {
                // power domain is assigned to base identifier
                let ident = arg.hierarchical_identifier.identifier.as_ref();
                if let Ok(symbol) = symbol_table::resolve(ident) {
                    self.push_expr_power_domain(
                        &symbol.found.kind,
                        arg.hierarchical_identifier.as_ref().into(),
                    );
                }
                self.check_expr_power_domains(&arg.semicolon.semicolon_token.token);
            }
        }
        Ok(())
    }

    fn inst_port_item(&mut self, arg: &InstPortItem) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.expr_power_domains.clear(),
            HandlerPoint::After => {
                if arg.inst_port_item_opt.is_none() {
                    if let Ok(symbol) = symbol_table::resolve(arg.identifier.as_ref()) {
                        self.push_expr_power_domain(
                            &symbol.found.kind,
                            arg.identifier.as_ref().into(),
                        );
                    }
                }
                let domain = self.check_expr_power_domains(&arg.identifier.identifier_token.token);
                let range: TokenRange = arg.identifier.as_ref().into();
                self.inst_power_domains
                    .insert(arg.identifier.identifier_token.token.text, (domain, range));
            }
        }
        Ok(())
    }

    fn inst_declaration(&mut self, arg: &InstDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.inst_power_domains.clear(),
            HandlerPoint::After => {
                let token = &arg.semicolon.semicolon_token.token;
                if let Ok(symbol) = symbol_table::resolve(arg.scoped_identifier.as_ref()) {
                    match &symbol.found.kind {
                        SymbolKind::Module(x) => {
                            let mut connection_table =
                                HashMap::<PowerDomain, (PowerDomain, TokenRange)>::new();
                            for x in &x.ports {
                                if let Some(connected) = self.inst_power_domains.get(&x.name()) {
                                    let port_domain = x.property().power_domain;
                                    if let Some(assigned) = connection_table.get(&port_domain) {
                                        if !assigned.0.compatible(&connected.0)
                                            && !unsafe_table::contains(token, Unsafe::Cdc)
                                        {
                                            self.errors.push(AnalyzerError::mismatch_power_domain(
                                                &connected.0.to_string(),
                                                &assigned.0.to_string(),
                                                self.text,
                                                &connected.1,
                                                &assigned.1,
                                            ));
                                        }
                                    } else {
                                        connection_table.insert(port_domain, *connected);
                                    }
                                }
                            }
                        }
                        SymbolKind::SystemVerilog => {
                            let mut prev: Option<(PowerDomain, TokenRange)> = None;
                            for curr in self.inst_power_domains.values() {
                                if let Some(prev) = prev {
                                    if !prev.0.compatible(&curr.0)
                                        && !unsafe_table::contains(token, Unsafe::Cdc)
                                    {
                                        self.errors.push(AnalyzerError::mismatch_power_domain(
                                            &curr.0.to_string(),
                                            &prev.0.to_string(),
                                            self.text,
                                            &curr.1,
                                            &prev.1,
                                        ));
                                    }
                                }
                                prev = Some(*curr);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    fn module_declaration(&mut self, arg: &ModuleDeclaration) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            let symbol = symbol_table::resolve(arg.identifier.as_ref()).unwrap();
            if let SymbolKind::Module(ref x) = symbol.found.kind {
                self.default_power = x.default_power;
            }
        }
        Ok(())
    }
}
