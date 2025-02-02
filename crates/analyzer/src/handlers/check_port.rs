use crate::analyzer_error::AnalyzerError;
use veryla_parser::veryla_grammar_trait::*;
use veryla_parser::veryla_walker::{Handler, HandlerPoint};
use veryla_parser::ParolError;

#[derive(Default)]
pub struct CheckPort<'a> {
    pub errors: Vec<AnalyzerError>,
    text: &'a str,
    point: HandlerPoint,
    in_function: bool,
    in_entity: bool,
    in_modport: bool,
}

impl<'a> CheckPort<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Handler for CheckPort<'_> {
    fn set_point(&mut self, p: HandlerPoint) {
        self.point = p;
    }
}

impl VerylaGrammarTrait for CheckPort<'_> {
    fn port_declaration_item(&mut self, arg: &PortDeclarationItem) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            if let PortDeclarationItemGroup::PortTypeConcrete(x) =
                arg.port_declaration_item_group.as_ref()
            {
                let x = x.port_type_concrete.as_ref();
                let direction = x.direction.as_ref();
                if let Direction::Inout(_) = direction {
                    let r#type = &x.array_type;
                    let is_tri = r#type
                        .scalar_type
                        .scalar_type_list
                        .iter()
                        .any(|x| matches!(x.type_modifier.as_ref(), TypeModifier::Tri(_)));

                    if !is_tri {
                        self.errors.push(AnalyzerError::missing_tri(
                            self.text,
                            &r#type.as_ref().into(),
                        ));
                    }
                }

                if let Some(x) = &x.port_type_concrete_opt0 {
                    let is_valid_port_default_value = match direction {
                        Direction::Input(_) => true,
                        Direction::Output(_) if !self.in_function => {
                            is_anonymous_expression(&x.port_default_value.expression)
                        }
                        _ => false,
                    };
                    if !is_valid_port_default_value {
                        self.errors.push(AnalyzerError::invalid_port_default_value(
                            &arg.identifier.identifier_token.to_string(),
                            &direction.to_string(),
                            self.text,
                            &x.port_default_value.expression.as_ref().into(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn direction(&mut self, arg: &Direction) -> Result<(), ParolError> {
        if let HandlerPoint::Before = self.point {
            match arg {
                Direction::Ref(x) => {
                    if !self.in_function {
                        self.errors.push(AnalyzerError::invalid_direction(
                            "ref",
                            self.text,
                            &x.r#ref.ref_token.token.into(),
                        ));
                    }
                }
                Direction::Modport(x) => {
                    if !self.in_entity || self.in_function {
                        self.errors.push(AnalyzerError::invalid_direction(
                            "modport",
                            self.text,
                            &x.modport.modport_token.token.into(),
                        ));
                    }
                }
                Direction::Import(x) => {
                    if !self.in_modport {
                        self.errors.push(AnalyzerError::invalid_direction(
                            "import",
                            self.text,
                            &x.import.import_token.token.into(),
                        ));
                    }
                }
                _ => (),
            }
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

    fn entity_declaration(&mut self, _arg: &EntityDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.in_entity = true,
            HandlerPoint::After => self.in_entity = false,
        }
        Ok(())
    }

    fn modport_declaration(&mut self, _arg: &ModportDeclaration) -> Result<(), ParolError> {
        match self.point {
            HandlerPoint::Before => self.in_modport = true,
            HandlerPoint::After => self.in_modport = false,
        }
        Ok(())
    }
}
