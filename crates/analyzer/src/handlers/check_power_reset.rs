use crate::analyzer_error::AnalyzerError;
use crate::evaluator::{Evaluated, Evaluator};
use crate::symbol::{SymbolKind, TypeKind};
use crate::symbol_table;
use veryla_parser::veryla_grammar_trait::*;
use veryla_parser::veryla_walker::{Handler, HandlerPoint};
use veryla_parser::ParolError;

#[derive(Default)]
pub struct CheckPowerReset<'a> {
    pub errors: Vec<AnalyzerError>,
    text: &'a str,
    point: HandlerPoint,
    in_sequence: bool,
    in_if_reset: bool,
    if_reset_brace: usize,
    if_reset_exist: bool,
    n_of_select: usize,
    default_power_exists: bool,
    default_reset_exists: bool,
    evaluator: Evaluator,
}

impl<'a> CheckPowerReset<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Handler for CheckPowerReset<'_> {
    fn set_point(&mut self, p: HandlerPoint) {
        self.point = p;
    }
}

impl VerylaGrammarTrait for CheckPowerReset<'_> {
    fn module_declaration(&mut self, arg: &ModuleDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => {
                if let Ok(found) = symbol_table::resolve(arg.identifier.as_ref()) {
                    if let SymbolKind::Module(x) = found.found.kind {
                        self.default_power_exists = x.default_power.is_some();
                        self.default_reset_exists = x.default_reset.is_some();
                    }
                }
            }
            HandlerPoint::After => {
                self.default_power_exists = false;
                self.default_reset_exists = false;
            }
        }
        Ok(())
    }

    fn l_brace(&mut self, _arg: &LBrace) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_if_reset {
                self.if_reset_brace += 1;
            }
        }
        Ok(())
    }

    fn r_brace(&mut self, _arg: &RBrace) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_if_reset {
                self.if_reset_brace -= 1;
                if self.if_reset_brace == 0 {
                    self.in_if_reset = false;
                }
            }
        }
        Ok(())
    }

    fn if_reset(&mut self, _arg: &IfReset) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            self.if_reset_exist = true;
            self.in_if_reset = true;
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

                // Check first if_reset when reset signel exists
                let if_reset_required = if let Some(ref x) = arg.sequence_declaration_opt {
                    if x.sequence_event_list.sequence_event_list_opt.is_some() {
                        if let Some(x) = arg.statement_block.statement_block_list.first() {
                            let x: Vec<_> = x.statement_block_group.as_ref().into();
                            if let Some(StatementBlockItem::Statement(x)) = x.first() {
                                !matches!(*x.statement, Statement::IfResetStatement(_))
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
                if if_reset_required {
                    self.errors
                        .push(AnalyzerError::missing_if_reset(self.text, &arg.into()));
                }

                self.in_sequence = true;
            }
            HandlerPoint::After => {
                // Check reset signal when if_reset exists
                if self.if_reset_exist {
                    let reset_signal_exists = if let Some(ref x) = arg.sequence_declaration_opt {
                        x.sequence_event_list.sequence_event_list_opt.is_some()
                    } else {
                        false
                    };
                    if !(self.default_reset_exists || reset_signal_exists) {
                        self.errors
                            .push(AnalyzerError::missing_reset_signal(self.text, &arg.into()));
                    }
                }

                self.in_sequence = false;
                self.if_reset_exist = false;
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

    fn sequence_reset(&mut self, arg: &SequenceReset) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.n_of_select = 0,
            HandlerPoint::After => {
                if let Ok(found) = symbol_table::resolve(arg.hierarchical_identifier.as_ref()) {
                    let symbol = found.found;
                    let valid_reset = match symbol.kind {
                        SymbolKind::Port(x) => {
                            let reset = x.r#type.clone().unwrap();
                            let n_of_select = reset.width.len() + reset.array.len();
                            match reset.kind {
                                TypeKind::Reset
                                | TypeKind::ResetAsyncHigh
                                | TypeKind::ResetAsyncLow
                                | TypeKind::ResetSyncHigh
                                | TypeKind::ResetSyncLow => n_of_select == self.n_of_select,
                                _ => false,
                            }
                        }
                        SymbolKind::Variable(x) => {
                            let reset = x.r#type;
                            let n_of_select = reset.width.len() + reset.array.len();
                            match reset.kind {
                                TypeKind::Reset
                                | TypeKind::ResetAsyncHigh
                                | TypeKind::ResetAsyncLow
                                | TypeKind::ResetSyncHigh
                                | TypeKind::ResetSyncLow => n_of_select == self.n_of_select,
                                _ => false,
                            }
                        }
                        _ => false,
                    };

                    if !valid_reset {
                        let token = &arg
                            .hierarchical_identifier
                            .identifier
                            .identifier_token
                            .token;
                        self.errors.push(AnalyzerError::invalid_reset(
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
            if self.in_if_reset {
                // Check to see right hand side of reset is const evaluable
                match self.evaluator.expression(&arg.expression) {
                    UnknownStatic | Fixed { .. } => (),
                    _ => {
                        self.errors
                            .push(AnalyzerError::invalid_reset_non_elaborative(
                                self.text,
                                &arg.expression.as_ref().into(),
                            ));
                    }
                }
            }
        }
        Ok(())
    }

    fn expression11(&mut self, arg: &Expression11) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if let Some(x) = &arg.expression11_opt {
                let src = self.evaluator.expression12(&arg.expression12);
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
                    CastingType::Reset(_)
                    | CastingType::ResetAsyncHigh(_)
                    | CastingType::ResetAsyncLow(_)
                    | CastingType::ResetSyncHigh(_)
                    | CastingType::ResetSyncLow(_) => {
                        if !src.is_reset() {
                            self.errors.push(AnalyzerError::invalid_cast(
                                "non-reset type",
                                "reset type",
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
