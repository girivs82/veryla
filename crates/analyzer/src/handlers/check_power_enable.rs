use crate::analyzer_error::AnalyzerError;
use crate::evaluator::{Evaluated, Evaluator};
use crate::symbol::{SymbolKind, TypeKind};
use crate::symbol_table;
use veryla_parser::veryla_grammar_trait::*;
use veryla_parser::veryla_walker::{Handler, HandlerPoint};
use veryla_parser::ParolError;

#[derive(Default)]
pub struct CheckPowerEnable<'a> {
    pub errors: Vec<AnalyzerError>,
    text: &'a str,
    point: HandlerPoint,
    in_sequence: bool,
    in_if_enable: bool,
    if_enable_brace: usize,
    if_enable_exist: bool,
    n_of_select: usize,
    default_power_exists: bool,
    default_enable_exists: bool,
    evaluator: Evaluator,
}

impl<'a> CheckPowerEnable<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Handler for CheckPowerEnable<'_> {
    fn set_point(&mut self, p: HandlerPoint) {
        self.point = p;
    }
}

impl VerylaGrammarTrait for CheckPowerEnable<'_> {
    fn entity_declaration(&mut self, arg: &EntityDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => {
                if let Ok(found) = symbol_table::resolve(arg.identifier.as_ref()) {
                    if let SymbolKind::Entity(x) = found.found.kind {
                        self.default_power_exists = x.default_power.is_some();
                        self.default_enable_exists = x.default_enable.is_some();
                    }
                }
            }
            HandlerPoint::After => {
                self.default_power_exists = false;
                self.default_enable_exists = false;
            }
        }
        Ok(())
    }

    fn l_brace(&mut self, _arg: &LBrace) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_if_enable {
                self.if_enable_brace += 1;
            }
        }
        Ok(())
    }

    fn r_brace(&mut self, _arg: &RBrace) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_if_enable {
                self.if_enable_brace -= 1;
                if self.if_enable_brace == 0 {
                    self.in_if_enable = false;
                }
            }
        }
        Ok(())
    }

    fn if_enable(&mut self, _arg: &IfEnable) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            self.if_enable_exist = true;
            self.in_if_enable = true;
        }
        Ok(())
    }

    fn sequence_declaration(&mut self, arg: &SequenceDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => {
                //  check if power signal exists
                let power_signal_exists = arg.sequence_declaration_opt.is_some();
                if !(self.default_power_exists || power_signal_exists) {
                    self.errors
                        .push(AnalyzerError::missing_power_signal(self.text, &arg.into()))
                }

                // Check first if_enable when enable signel exists
                let if_enable_required = if let Some(ref x) = arg.sequence_declaration_opt {
                    if x.sequence_event_list.sequence_event_list_opt.is_some() {
                        if let Some(x) = arg.statement_block.statement_block_list.first() {
                            let x: Vec<_> = x.statement_block_group.as_ref().into();
                            if let Some(StatementBlockItem::Statement(x)) = x.first() {
                                !matches!(*x.statement, Statement::IfEnableStatement(_))
                            } else {
                                true
                            }
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                if if_enable_required {
                    self.errors
                        .push(AnalyzerError::missing_if_enable(self.text, &arg.into()));
                }

                self.in_sequence = true;
            }
            HandlerPoint::After => {
                // Check enable signal when if_enable exists
                if self.if_enable_exist {
                    let enable_signal_exists = if let Some(ref x) = arg.sequence_declaration_opt {
                        x.sequence_event_list.sequence_event_list_opt.is_some()
                    } else {
                        false
                    };
                    if !(self.default_enable_exists || enable_signal_exists) {
                        self.errors
                            .push(AnalyzerError::missing_enable_signal(self.text, &arg.into()));
                    }
                }

                self.in_sequence = false;
                self.if_enable_exist = false;
            }
        }
        Ok(())
    }

    fn sequence_power(&mut self, arg: &SequencePower) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.n_of_select = 0,
            HandlerPoint::After => {
                if let Ok(found) = symbol_table::resolve(arg.hierarchical_identifier.as_ref()) {
                    let symbol = found.found;
                    let valid_power = match symbol.kind {
                        SymbolKind::Port(x) => {
                            let power = x.r#type.clone().unwrap();
                            let n_of_select = power.width.len() + power.array.len();
                            match power.kind {
                                TypeKind::Power
                                | TypeKind::PowerPosedge
                                | TypeKind::PowerNegedge => n_of_select == self.n_of_select,
                                _ => false,
                            }
                        }
                        SymbolKind::Variable(x) => {
                            let power = x.r#type;
                            let n_of_select = power.width.len() + power.array.len();
                            match power.kind {
                                TypeKind::Power
                                | TypeKind::PowerPosedge
                                | TypeKind::PowerNegedge => n_of_select == self.n_of_select,
                                _ => false,
                            }
                        }
                        _ => false,
                    };

                    if !valid_power {
                        let token = &arg
                            .hierarchical_identifier
                            .identifier
                            .identifier_token
                            .token;
                        self.errors.push(AnalyzerError::invalid_power(
                            &token.to_string(),
                            self.text,
                            &arg.hierarchical_identifier.as_ref().into(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn sequence_enable(&mut self, arg: &SequenceEnable) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.n_of_select = 0,
            HandlerPoint::After => {
                if let Ok(found) = symbol_table::resolve(arg.hierarchical_identifier.as_ref()) {
                    let symbol = found.found;
                    let valid_enable = match symbol.kind {
                        SymbolKind::Port(x) => {
                            let enable = x.r#type.clone().unwrap();
                            let n_of_select = enable.width.len() + enable.array.len();
                            match enable.kind {
                                TypeKind::Enable
                                | TypeKind::EnableHigh
                                | TypeKind::EnableLow => n_of_select == self.n_of_select,
                                _ => false,
                            }
                        }
                        SymbolKind::Variable(x) => {
                            let enable = x.r#type;
                            let n_of_select = enable.width.len() + enable.array.len();
                            match enable.kind {
                                TypeKind::Enable
                                | TypeKind::EnableHigh
                                | TypeKind::EnableLow => n_of_select == self.n_of_select,
                                _ => false,
                            }
                        }
                        _ => false,
                    };

                    if !valid_enable {
                        let token = &arg
                            .hierarchical_identifier
                            .identifier
                            .identifier_token
                            .token;
                        self.errors.push(AnalyzerError::invalid_enable(
                            &token.to_string(),
                            self.text,
                            &arg.hierarchical_identifier.as_ref().into(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn select(&mut self, _arg: &Select) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            self.n_of_select += 1;
        }
        Ok(())
    }

    fn dot(&mut self, _arg: &Dot) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            self.n_of_select = 0;
        }
        Ok(())
    }

    fn assignment(&mut self, arg: &Assignment) -> Result<(), ParolError> {
        use Evaluated::*;
        if let HandlerPoint::Before = self.point {
            if self.in_if_enable {
                // Check to see right hand side of enable is const evaluable
                match self.evaluator.expression(&arg.expression) {
                    UnknownStatic | Fixed { .. } => (),
                    _ => {
                        self.errors
                            .push(AnalyzerError::invalid_enable_non_elaborative(
                                self.text,
                                &arg.expression.as_ref().into(),
                            ));
                    }
                }
            }
        }
        Ok(())
    }

    fn expression12(&mut self, arg: &Expression12) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if let Some(x) = &arg.expression12_opt {
                let src = self.evaluator.expression13(&arg.expression13);
                match x.casting_type.as_ref() {
                    CastingType::Power(_)
                    | CastingType::PowerPosedge(_)
                    | CastingType::PowerNegedge(_) => {
                        if !src.is_power() {
                            self.errors.push(AnalyzerError::invalid_cast(
                                "non-power type",
                                "power type",
                                self.text,
                                &arg.into(),
                            ));
                        }
                    }
                    CastingType::Enable(_)
                    | CastingType::EnableHigh(_)
                    | CastingType::EnableLow(_) => {
                        if !src.is_enable() {
                            self.errors.push(AnalyzerError::invalid_cast(
                                "non-enable type",
                                "enable type",
                                self.text,
                                &arg.into(),
                            ));
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
}
