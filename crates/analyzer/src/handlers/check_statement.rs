use crate::analyzer_error::AnalyzerError;
use veryla_parser::veryla_grammar_trait::*;
use veryla_parser::veryla_walker::{Handler, HandlerPoint};
use veryla_parser::ParolError;

#[derive(Default)]
pub struct CheckStatement<'a> {
    pub errors: Vec<AnalyzerError>,
    text: &'a str,
    point: HandlerPoint,
    in_sequence: bool,
    in_always_comb: bool,
    in_function: bool,
    in_initial: bool,
    in_final: bool,
    statement_depth_in_sequence: usize,
    statement_depth_in_loop: usize,
}

impl<'a> CheckStatement<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Handler for CheckStatement<'_> {
    fn set_point(&mut self, p: HandlerPoint) {
        self.point = p;
    }
}

impl VerylaGrammarTrait for CheckStatement<'_> {
    fn statement(&mut self, _arg: &Statement) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            self.statement_depth_in_sequence += 1;
        }
        Ok(())
    }

    fn assignment(&mut self, arg: &Assignment) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.in_initial || self.in_final {
                let token = match &*arg.assignment_group {
                    AssignmentGroup::Equ(x) => &x.equ.equ_token.token,
                    AssignmentGroup::AssignmentOperator(x) => {
                        &x.assignment_operator.assignment_operator_token.token
                    }
                };
                self.errors.push(AnalyzerError::invalid_statement(
                    "assignment",
                    self.text,
                    &token.into(),
                ));
            }
        }
        Ok(())
    }

    fn if_enable_statement(&mut self, arg: &IfEnableStatement) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if !self.in_sequence {
                self.errors.push(AnalyzerError::invalid_statement(
                    "if_enable",
                    self.text,
                    &arg.if_enable.if_enable_token.token.into(),
                ));
            }

            if self.in_sequence && self.statement_depth_in_sequence != 1 {
                self.errors.push(AnalyzerError::invalid_statement(
                    "if_enable",
                    self.text,
                    &arg.if_enable.if_enable_token.token.into(),
                ));
            }
        }
        Ok(())
    }

    fn return_statement(&mut self, arg: &ReturnStatement) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if !self.in_function {
                self.errors.push(AnalyzerError::invalid_statement(
                    "return",
                    self.text,
                    &arg.r#return.return_token.token.into(),
                ));
            }
        }
        Ok(())
    }

    fn break_statement(&mut self, arg: &BreakStatement) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if self.statement_depth_in_loop == 0 {
                self.errors.push(AnalyzerError::invalid_statement(
                    "break",
                    self.text,
                    &arg.r#break.break_token.token.into(),
                ));
            }
        }
        Ok(())
    }

    fn for_statement(&mut self, _arg: &ForStatement) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.statement_depth_in_loop += 1,
            HandlerPoint::After => self.statement_depth_in_loop -= 1,
        }
        Ok(())
    }

    fn sequence_declaration(&mut self, _arg: &SequenceDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => {
                self.in_sequence = true;
                self.statement_depth_in_sequence = 0;
            }
            HandlerPoint::After => self.in_sequence = false,
        }
        Ok(())
    }

    fn always_comb_declaration(&mut self, _arg: &AlwaysCombDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.in_always_comb = true,
            HandlerPoint::After => self.in_always_comb = false,
        }
        Ok(())
    }

    fn initial_declaration(&mut self, _arg: &InitialDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.in_initial = true,
            HandlerPoint::After => self.in_initial = false,
        }
        Ok(())
    }

    fn final_declaration(&mut self, _arg: &FinalDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.in_final = true,
            HandlerPoint::After => self.in_final = false,
        }
        Ok(())
    }

    fn function_declaration(&mut self, _arg: &FunctionDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.in_function = true,
            HandlerPoint::After => self.in_function = false,
        }
        Ok(())
    }
}
