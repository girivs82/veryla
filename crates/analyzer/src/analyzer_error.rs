use miette::{self, Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
use veryla_parser::veryla_token::TokenRange;

#[derive(Error, Diagnostic, Debug)]
pub enum AnalyzerError {
    #[diagnostic(severity(Error), code(anonymous_identifier_usage), help(""), url(""))]
    #[error("Anonymous identifier can't be placed at here")]
    AnonymousIdentifierUsage {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(call_non_function),
        help("remove call to non-function symbol"),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#call_non_function"
        )
    )]
    #[error("Calling non-function symbol \"{identifier}\" which has kind \"{kind}\"")]
    CallNonFunction {
        identifier: String,
        kind: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(cyclice_type_dependency),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#cyclic_type_dependency")
    )]
    #[error("Cyclic dependency between {start} and {end}")]
    CyclicTypeDependency {
        start: String,
        end: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(duplicated_identifier),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#duplicated_identifier")
    )]
    #[error("{identifier} is duplicated")]
    DuplicatedIdentifier {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(multiple_assignment),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#multiple_assignment")
    )]
    #[error("{identifier} is assigned in multiple procedural blocks or assignment statements")]
    MultipleAssignment {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
        #[label("Assigned")]
        assign_pos0: SourceSpan,
        #[label("Assigned too")]
        assign_pos1: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_allow),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_allow")
    )]
    #[error("{identifier} can't be allowed")]
    InvalidAllow {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_assignment),
        help("remove the assignment"),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_assignment"
        )
    )]
    #[error("{identifier} can't be assigned because it is {kind}")]
    InvalidAssignment {
        identifier: String,
        kind: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_assignment_to_const),
        help("remove the assignment"),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_assignment_to_const"
        )
    )]
    #[error("{identifier} can't be assigned because it is const")]
    InvalidAssignmentToConst {
        identifier: String,
        kind: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_direction),
        help("remove {kind} direction"),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_direction"
        )
    )]
    #[error("{kind} direction can't be placed at here")]
    InvalidDirection {
        kind: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_factor),
        help("remove {kind} from expression"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_factor")
    )]
    #[error("{identifier} of kind \"{kind}\" cannot be used as a factor in an expression")]
    InvalidFactor {
        identifier: String,
        kind: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(invalid_identifier),
        help("follow naming rule"),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_identifier"
        )
    )]
    #[error("{identifier} violate \"{rule}\" naming rule")]
    InvalidIdentifier {
        identifier: String,
        rule: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_import),
        help("fix import item"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_import")
    )]
    #[error("This item can't be imported")]
    InvalidImport {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_lsb),
        help("remove lsb"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_lsb")
    )]
    #[error("lsb can't be placed at here")]
    InvalidLsb {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_msb),
        help("remove msb"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_msb")
    )]
    #[error("msb can't be placed at here")]
    InvalidMsb {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_number_character),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_number_character")
    )]
    #[error("{kind} number can't contain {cause}")]
    InvalidNumberCharacter {
        cause: char,
        kind: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_statement),
        help("remove {kind} statement"),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_statement"
        )
    )]
    #[error("{kind} statement can't be placed at here")]
    InvalidStatement {
        kind: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_power),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_power")
    )]
    #[error("#{identifier} can't be used as a power because it is not 'power' type nor a single bit signal")]
    InvalidPower {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_modport_variable_item),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_modport_variable_item")
    )]
    #[error("#{identifier} is not a variable")]
    InvalidModportVariableItem {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_modport_function_item),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_modport_function_item")
    )]
    #[error("#{identifier} is not a function")]
    InvalidModportFunctionItem {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(severity(Error), code(invalid_port_default_value), help(""), url(""))]
    #[error("#{direction} port #{identifier} cannot have a port default value")]
    InvalidPortDefaultValue {
        identifier: String,
        direction: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_enable),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_enable")
    )]
    #[error("#{identifier} can't be used as a enable because it is not 'enable' type nor a single bit signal")]
    InvalidReset {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_enable_non_elaborative),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_enable_non_elaborative")
    )]
    #[error("Reset-value cannot be used because it is not evaluable at elaboration time")]
    InvalidResetNonElaborative {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_case_condition_non_elaborative),
        help(""),
        url("")
    )]
    #[error("Case condition value cannot be used because it is not evaluable at elaboration time")]
    InvalidCaseConditionNonElaborative {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(severity(Error), code(invalid_cast), help(""), url(""))]
    #[error("Casting from {from} to {to} is incompatible")]
    InvalidCast {
        from: String,
        to: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_test),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_test")
    )]
    #[error("test is invalid because {cause}")]
    InvalidTest {
        cause: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(incompat_proto),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#incompat_proto")
    )]
    #[error("{identifier} is incompatible with {proto} because {cause}")]
    IncompatProto {
        identifier: String,
        proto: String,
        cause: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(missing_default_argument),
        help("give default argument"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_default_argument")
    )]
    #[error("missing default argument for parameter {identifier}")]
    MissingDefaultArgument {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(mismatch_function_arity),
        help("fix function arguments"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#mismatch_function_arity")
    )]
    #[error("function \"{name}\" has {arity} arguments, but {args} arguments are supplied")]
    MismatchFunctionArity {
        name: String,
        arity: usize,
        args: usize,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(mismatch_generics_arity),
        help("fix generics arguments"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#mismatch_generics_arity")
    )]
    #[error(
        "generics \"{name}\" has {arity} generic arguments, but {args} arguments are supplied"
    )]
    MismatchGenericsArity {
        name: String,
        arity: usize,
        args: usize,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(mismatch_attribute_args),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#mismatch_attribute_args")
    )]
    #[error("Arguments of \"{name}\" is expected to \"{expected}\"")]
    MismatchAttributeArgs {
        name: String,
        expected: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(mismatch_type),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#mismatch_type")
    )]
    #[error("\"{name}\" is expected to \"{expected}\", but it is \"{actual}\"")]
    MismatchType {
        name: String,
        expected: String,
        actual: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(mismatch_power_domain),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#mismatch_power_domain")
    )]
    #[error("Power domain crossing is detected")]
    MismatchPowerDomain {
        power_domain: String,
        other_domain: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("power domain {power_domain}")]
        error_location: SourceSpan,
        #[label("power domain {other_domain}")]
        other_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(missing_if_enable),
        help("add if_enable statement"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_if_enable")
    )]
    #[error("if_enable statement is required for sequence with enable signal")]
    MissingIfReset {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(missing_port),
        help("add \"{port}\" port"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_port")
    )]
    #[error("entity \"{name}\" has \"{port}\", but it is not connected")]
    MissingPort {
        name: String,
        port: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(missing_power_signal),
        help("add power port"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_power_signal")
    )]
    #[error("power signal is required for sequence statement")]
    MissingPowerSignal {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(missing_enable_signal),
        help("add enable port"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_enable_signal")
    )]
    #[error("enable signal is required for sequence with if_enable statement")]
    MissingResetSignal {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(missing_enable_statement),
        help("add enable statement"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_enable_statement")
    )]
    #[error("{name} is not enable in if_enable statement")]
    MissingResetStatement {
        name: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
        #[label("Not enable")]
        enable: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(missing_tri),
        help("add tri type modifier"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_tri")
    )]
    #[error("tri type modifier is required at inout port")]
    MissingTri {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(missing_power_domain),
        help("add power domain annotation"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#missing_power_domain")
    )]
    #[error("power domain annotation is required when there are multiple powers")]
    MissingPowerDomain {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(sv_keyword_usage),
        help("Change the identifier to a non-SystemVerilog keyword"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#sv_keyword_usage")
    )]
    #[error("SystemVerilog keyword may not be used as identifier")]
    SvKeywordUsage {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(sv_with_implicit_enable),
        help("Use types with explicit polarity like `enable_low`"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#sv_with_implicit_enable")
    )]
    #[error("Reset type with implicit synchronisity and polarity can't be connected to SystemVerilog entity")]
    SvWithImplicitEnable {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_enum_encoding),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_enum_encoding")
    )]
    #[error("{identifier} is not valid enum encoding")]
    InvalidEnumEncoding {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(invalid_cond_type),
        help(""),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#invalid_cond_type"
        )
    )]
    #[error("{identifier} is not valid condition type")]
    InvalidCondType {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(too_large_enum_variant),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#too_large_enum_variant")
    )]
    #[error("The value of enum variant {identifier} is {value}, it is can't be represented by {width} bits")]
    TooLargeEnumVariant {
        identifier: String,
        value: isize,
        width: usize,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unevaluatable_enum_variant_value),
        help(""),
        url("")
    )]
    #[error("The value of enum variant {identifier} cannot be evaluated")]
    UnevaluatableEnumVariant {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(severity(Error), code(invalid_enum_variant_value), help(""), url(""))]
    #[error("The value of enum variant {identifier} is not encoded value by {encoding}")]
    InvalidEnumVariant {
        identifier: String,
        encoding: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(too_large_number),
        help("increase bit width"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#too_large_number")
    )]
    #[error("number is over the maximum size of {width} bits")]
    TooLargeNumber {
        width: usize,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(too_much_enum_variant),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#too_much_enum_variant")
    )]
    #[error(
        "enum {identifier} has {number} variants, they are can't be represented by {width} bits"
    )]
    TooMuchEnumVariant {
        identifier: String,
        number: usize,
        width: usize,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(undefined_identifier),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#undefined_identifier")
    )]
    #[error("{identifier} is undefined")]
    UndefinedIdentifier {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(referring_package_before_definition),
        help("change order of package definitions"),
        url("")
    )]
    #[error("pakcakge {identifier} is referred before it is defined.")]
    ReferringPackageBeforeDefinition {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unresolvable_generic_argument),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unresolvable_generic_argument")
    )]
    #[error("{identifier} can't be resolved from the definition of generics")]
    UnresolvableGenericArgument {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
        #[label("Definition")]
        definition_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_attribute),
        help(""),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_attribute"
        )
    )]
    #[error("\"{name}\" is not valid attribute")]
    UnknownAttribute {
        name: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_embed_lang),
        help(""),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_embed_lang"
        )
    )]
    #[error("\"{name}\" is not valid embed language")]
    UnknownEmbedLang {
        name: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_embed_way),
        help(""),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_embed_way"
        )
    )]
    #[error("\"{name}\" is not valid embed way")]
    UnknownEmbedWay {
        name: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_include_way),
        help(""),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_include_way"
        )
    )]
    #[error("\"{name}\" is not valid include way")]
    UnknownIncludeWay {
        name: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_member),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_member")
    )]
    #[error("\"{name}\" doesn't have member \"{member}\"")]
    UnknownMember {
        name: String,
        member: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_unsafe),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_unsafe")
    )]
    #[error("\"{name}\" is not valid unsafe identifier")]
    UnknownUnsafe {
        name: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(private_member),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#private_member")
    )]
    #[error("\"{name}\" is private member")]
    PrivateMember {
        name: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_msb),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_msb")
    )]
    #[error("resolving msb is failed")]
    UnknownMsb {
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_port),
        help("remove \"{port}\" port"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_port")
    )]
    #[error("entity \"{name}\" doesn't have port \"{port}\", but it is connected")]
    UnknownPort {
        name: String,
        port: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(unknown_param),
        help("remove \"{param}\" param"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unknown_param")
    )]
    #[error("entity \"{name}\" doesn't have param \"{param}\", but it is overrided")]
    UnknownParam {
        name: String,
        param: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(unused_variable),
        help("add prefix `_` to unused variable name"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unused_variable")
    )]
    #[error("{identifier} is unused")]
    UnusedVariable {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(unused_return),
        help("add variable assignment for function return"),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unused_return")
    )]
    #[error("return value of {identifier} is unused")]
    UnusedReturn {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(unassign_variable),
        help(""),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#unassign_variable"
        )
    )]
    #[error("{identifier} is unassigned")]
    UnassignVariable {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Warning),
        code(uncovered_branch),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#uncovered_branch")
    )]
    #[error("{identifier} is not covered by all branches, it causes latch generation")]
    UncoveredBranch {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
        #[label("Uncovered")]
        uncovered: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(reserved_identifier),
        help("prefix `__` can't be used"),
        url(
            "https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#reserved_identifier"
        )
    )]
    #[error("{identifier} is reverved for compiler usage")]
    ReservedIdentifier {
        identifier: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(include_failure),
        help(""),
        url("https://doc.veryl-lang.org/book/07_appendix/02_semantic_error.html#include_failure")
    )]
    #[error("\"{name}\" can't be read because \"{cause}\"")]
    IncludeFailure {
        name: String,
        cause: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },

    #[diagnostic(
        severity(Error),
        code(wrong_seperator),
        help("replace valid separator \"{valid_separator}\""),
        url("")
    )]
    #[error("separator \"{separator}\" can't be used at here")]
    WrongSeparator {
        separator: String,
        valid_separator: String,
        #[source_code]
        input: NamedSource<String>,
        #[label("Error location")]
        error_location: SourceSpan,
    },
}

impl AnalyzerError {
    fn named_source(source: &str, token: &TokenRange) -> NamedSource<String> {
        NamedSource::new(token.beg.source.to_string(), source.to_string())
    }

    pub fn anonymous_identifier_usage(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::AnonymousIdentifierUsage {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn call_non_function(
        identifier: &str,
        kind: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::CallNonFunction {
            identifier: identifier.into(),
            kind: kind.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn cyclic_type_dependency(
        source: &str,
        start: &str,
        end: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::CyclicTypeDependency {
            start: start.into(),
            end: end.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn duplicated_identifier(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::DuplicatedIdentifier {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn multiple_assignment(
        identifier: &str,
        source: &str,
        token: &TokenRange,
        assign_pos0: &TokenRange,
        assign_pos1: &TokenRange,
    ) -> Self {
        AnalyzerError::MultipleAssignment {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
            assign_pos0: assign_pos0.into(),
            assign_pos1: assign_pos1.into(),
        }
    }

    pub fn invalid_allow(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidAllow {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_assignment(
        identifier: &str,
        source: &str,
        kind: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidAssignment {
            identifier: identifier.into(),
            kind: kind.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_assignment_to_const(
        identifier: &str,
        source: &str,
        kind: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidAssignmentToConst {
            identifier: identifier.into(),
            kind: kind.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_direction(kind: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidDirection {
            kind: kind.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_factor(identifier: &str, kind: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidFactor {
            identifier: identifier.to_string(),
            kind: kind.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_identifier(
        identifier: &str,
        rule: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidIdentifier {
            identifier: identifier.to_string(),
            rule: rule.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_import(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidImport {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_lsb(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidLsb {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_msb(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidMsb {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_number_character(
        cause: char,
        kind: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidNumberCharacter {
            cause,
            kind: kind.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_statement(kind: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidStatement {
            kind: kind.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_power(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidPower {
            identifier: identifier.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_enable(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidReset {
            identifier: identifier.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_enable_non_elaborative(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidResetNonElaborative {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_case_condition_non_elaborative(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidCaseConditionNonElaborative {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_cast(from: &str, to: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidCast {
            from: from.into(),
            to: to.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_test(cause: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidTest {
            cause: cause.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_modport_variable_item(
        identifier: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidModportVariableItem {
            identifier: identifier.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_modport_function_item(
        identifier: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidModportFunctionItem {
            identifier: identifier.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_port_default_value(
        identifier: &str,
        direction: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidPortDefaultValue {
            identifier: identifier.into(),
            direction: direction.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn incompat_proto(
        identifier: &str,
        proto: &str,
        cause: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::IncompatProto {
            identifier: identifier.into(),
            proto: proto.into(),
            cause: cause.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn missing_default_argument(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::MissingDefaultArgument {
            identifier: identifier.into(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn mismatch_function_arity(
        name: &str,
        arity: usize,
        args: usize,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::MismatchFunctionArity {
            name: name.to_string(),
            arity,
            args,
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn mismatch_generics_arity(
        name: &str,
        arity: usize,
        args: usize,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::MismatchGenericsArity {
            name: name.to_string(),
            arity,
            args,
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn mismatch_type(
        name: &str,
        expected: &str,
        actual: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::MismatchType {
            name: name.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn mismatch_power_domain(
        power_domain: &str,
        other_domain: &str,
        source: &str,
        token: &TokenRange,
        other_token: &TokenRange,
    ) -> Self {
        AnalyzerError::MismatchPowerDomain {
            power_domain: power_domain.to_string(),
            other_domain: other_domain.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
            other_location: other_token.into(),
        }
    }

    pub fn missing_power_signal(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::MissingPowerSignal {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn missing_if_enable(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::MissingIfReset {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn missing_enable_signal(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::MissingResetSignal {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn missing_enable_statement(
        name: &str,
        source: &str,
        token: &TokenRange,
        enable: &TokenRange,
    ) -> Self {
        AnalyzerError::MissingResetStatement {
            name: name.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
            enable: enable.into(),
        }
    }

    pub fn missing_tri(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::MissingTri {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn missing_power_domain(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::MissingPowerDomain {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn mismatch_attribute_args(
        name: &str,
        expected: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::MismatchAttributeArgs {
            name: name.to_string(),
            expected: expected.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn missing_port(name: &str, port: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::MissingPort {
            name: name.to_string(),
            port: port.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn sv_keyword_usage(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::SvKeywordUsage {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn sv_with_implicit_enable(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::SvWithImplicitEnable {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_enum_encoding(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidEnumEncoding {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_cond_type(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::InvalidCondType {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn too_large_enum_variant(
        identifier: &str,
        value: isize,
        width: usize,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::TooLargeEnumVariant {
            identifier: identifier.to_string(),
            value,
            width,
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unevaluatable_enum_variant(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnevaluatableEnumVariant {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn invalid_enum_variant_value(
        identifier: &str,
        encoding: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::InvalidEnumVariant {
            identifier: identifier.to_string(),
            encoding: encoding.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn too_large_number(width: usize, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::TooLargeNumber {
            width,
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn too_much_enum_variant(
        identifier: &str,
        number: usize,
        width: usize,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::TooMuchEnumVariant {
            identifier: identifier.to_string(),
            number,
            width,
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn undefined_identifier(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UndefinedIdentifier {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn referring_package_before_definition(
        identifier: &str,
        source: &str,
        token: &TokenRange,
    ) -> Self {
        AnalyzerError::ReferringPackageBeforeDefinition {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unresolvable_generic_argument(
        identifier: &str,
        source: &str,
        token: &TokenRange,
        definition_token: &TokenRange,
    ) -> Self {
        AnalyzerError::UnresolvableGenericArgument {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
            definition_location: definition_token.into(),
        }
    }

    pub fn unknown_attribute(name: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownAttribute {
            name: name.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_embed_lang(name: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownEmbedLang {
            name: name.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_embed_way(name: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownEmbedWay {
            name: name.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_include_way(name: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownIncludeWay {
            name: name.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_member(name: &str, member: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownMember {
            name: name.to_string(),
            member: member.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_unsafe(name: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownUnsafe {
            name: name.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn private_member(name: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::PrivateMember {
            name: name.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_msb(source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownMsb {
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_port(name: &str, port: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownPort {
            name: name.to_string(),
            port: port.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unknown_param(name: &str, param: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnknownParam {
            name: name.to_string(),
            param: param.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unused_variable(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnusedVariable {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unused_return(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnusedReturn {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn unassign_variable(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::UnassignVariable {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn uncovered_branch(
        identifier: &str,
        source: &str,
        token: &TokenRange,
        uncovered: &TokenRange,
    ) -> Self {
        AnalyzerError::UncoveredBranch {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
            uncovered: uncovered.into(),
        }
    }

    pub fn reserved_identifier(identifier: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::ReservedIdentifier {
            identifier: identifier.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn include_failure(name: &str, cause: &str, source: &str, token: &TokenRange) -> Self {
        AnalyzerError::IncludeFailure {
            name: name.to_string(),
            cause: cause.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }

    pub fn wrong_seperator(separator: &str, source: &str, token: &TokenRange) -> Self {
        let valid_separator = if separator == "." { "::" } else { "." };
        AnalyzerError::WrongSeparator {
            separator: separator.to_string(),
            valid_separator: valid_separator.to_string(),
            input: AnalyzerError::named_source(source, token),
            error_location: token.into(),
        }
    }
}
