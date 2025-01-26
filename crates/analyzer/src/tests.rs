use crate::{symbol_table, Analyzer, AnalyzerError};
use veryla_metadata::Metadata;
use veryla_parser::Parser;

#[track_caller]
fn analyze(code: &str) -> Vec<AnalyzerError> {
    symbol_table::clear();

    let metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();
    let parser = Parser::parse(&code, &"").unwrap();
    let analyzer = Analyzer::new(&metadata);

    let mut errors = vec![];
    errors.append(&mut analyzer.analyze_pass1(&"prj", &code, &"", &parser.veryla));
    Analyzer::analyze_post_pass1();
    errors.append(&mut analyzer.analyze_pass2(&"prj", &code, &"", &parser.veryla));
    errors.append(&mut analyzer.analyze_pass3(&"prj", &code, &"", &parser.veryla));
    dbg!(&errors);
    errors
}

#[test]
fn power_check() {
    let code = r#"
    entity EntityA (
        pwr: input power
    ) {
        var a: logic;
        sequence (pwr) {
            a = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityB (
        pwr_a: input `_a power<2>,
        pwr_b: input `_b power[2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a[POS]) {
            a = 0;
        }
        sequence (pwr_b[POS]) {
            b = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityC (
        pwr_a: input `_a power<2, 2>,
        pwr_b: input `_b power[2, 2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a[POS][POS]) {
            a = 0;
        }
        sequence (pwr_b[POS][POS]) {
            b = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityD (
        pwr: input logic
    ) {
        var a: logic;
        sequence (pwr) {
            a = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidPower { .. }));

    let code = r#"
    entity EntityE (
        pwr_a: input `_a power<2>,
        pwr_b: input `_b power[2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a) {
            a = 0;
        }
        sequence (pwr_b[POS]) {
            b = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidPower { .. }));

    let code = r#"
    entity EntityF (
        pwr_a: input `_a power<2>,
        pwr_b: input `_b power[2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a[POS]) {
            a = 0;
        }
        sequence (pwr_b) {
            b = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidPower { .. }));

    let code = r#"
    entity EntityG (
        pwr_a: input `_a power<2, 2>,
        pwr_b: input `_b power[2, 2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a[POS]) {
            a = 0;
        }
        sequence (pwr_b[POS][POS]) {
            b = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidPower { .. }));

    let code = r#"
    entity EntityH (
        pwr_a: input `_a power<2, 2>,
        pwr_b: input `_b power[2, 2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a[POS][POS]) {
            a = 0;
        }
        sequence (pwr_b[POS]) {
            b = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidPower { .. }));
}

#[test]
fn enable_check() {
    let code = r#"
    entity EntityA (
        pwr: input power,
        en: input enable
    ) {
        var a: logic;
        sequence (pwr, en) {
            if_enable {
                a = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityB (
        pwr_a: input `_a power,
        rst_a: input `_a enable<2>,
        pwr_b: input `_b power,
        rst_b: input `_b enable[2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a, rst_a[POS]) {
            if_enable {
                a = 0;
            }
        }
        sequence (pwr_b, rst_b[POS]) {
            if_enable {
                b = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityC (
        pwr_a: input `_a power,
        rst_a: input `_a enable<2, 2>,
        pwr_b: input `_b power,
        rst_b: input `_b enable[2, 2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a, rst_a[POS][POS]) {
            if_enable {
                a = 0;
            }
        }
        sequence (pwr_b, rst_b[POS][POS]) {
            if_enable {
                b = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityD (
        pwr: input power,
        en: input logic
    ) {
        var a: logic;
        sequence (pwr, en) {
            if_enable {
                a = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidReset { .. }));

    let code = r#"
    entity EntityE (
        pwr_a: input `_a power,
        rst_a: input `_a enable<2>,
        pwr_b: input `_b power,
        rst_b: input `_b enable[2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a, rst_a) {
            if_enable {
                a = 0;
            }
        }
        sequence (pwr_b, rst_b[POS]) {
            if_enable {
                b = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidReset { .. }));

    let code = r#"
    entity EntityF (
        pwr_a: input `_a power,
        rst_a: input `_a enable<2>,
        pwr_b: input `_b power,
        rst_b: input `_b enable[2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a, rst_a[POS]) {
            if_enable {
                a = 0;
            }
        }
        sequence (pwr_b, rst_b) {
            if_enable {
                b = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidReset { .. }));

    let code = r#"
    entity EntityG (
        pwr_a: input `_a power,
        rst_a: input `_a enable<2, 2>,
        pwr_b: input `_b power,
        rst_b: input `_b enable[2, 2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a, rst_a[POS]) {
            if_enable {
                a = 0;
            }
        }
        sequence (pwr_b, rst_b[POS][POS]) {
            if_enable {
                b = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidReset { .. }));

    let code = r#"
    entity EntityH (
        pwr_a: input `_a power,
        rst_a: input `_a enable<2, 2>,
        pwr_b: input `_b power,
        rst_b: input `_b enable[2, 2]
    ) {
        const POS: u32 = 0;
        var a: `_a logic;
        var b: `_b logic;
        sequence (pwr_a, rst_a[POS][POS]) {
            if_enable {
                a = 0;
            }
        }
        sequence (pwr_b, rst_b[POS]) {
            if_enable {
                b = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidReset { .. }));
}

#[test]
fn power_connection_check() {
    let code = r#"
    entity EntityA (
        pwr: input logic
    ) {
        inst u: EntityB (
            pwr,
        );
    }

    entity EntityB (
        pwr: input power
    ) {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));
}

#[test]
fn enable_connection_check() {
    let code = r#"
    entity EntityA (
        pwr: input logic
    ) {
        inst u: EntityB (
            pwr,
        );
    }

    entity EntityB (
        pwr: input enable
    ) {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));
}

#[test]
fn cyclic_type_dependency() {
    let code = r#"
    entity EntityA {
        struct StructA {
            memberA: StructA,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::CyclicTypeDependency { .. }
    ));

    let code = r#"
    entity EntityB {
        inst u: EntityC;
    }
    entity EntityC {
        inst u: EntityB;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::CyclicTypeDependency { .. }
    ));

    let code = r#"
    package PackageA {
        import PackageA::*;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::CyclicTypeDependency { .. }
    ));
}

#[test]
fn duplicated_identifier() {
    let code = r#"
    entity EntityA {
        const a: u32 = 1;
        const a: u32 = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::DuplicatedIdentifier { .. }
    ));
}

#[test]
fn multiple_assignment() {
    let code = r#"
    entity EntityA {
        var a: logic;

        assign a = 1;
        always_comb {
            a = 1;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MultipleAssignment { .. }
    ));
}

#[test]
fn invalid_allow() {
    let code = r#"
    entity EntityA {
        #[allow(dummy_name)]
        var a: logic;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidAllow { .. }));
}

#[test]
fn invalid_assignment() {
    let code = r#"
    entity EntityA (
        a: input logic,
    ) {
        assign a = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidAssignment { .. }));

    let code = r#"
    entity EntityB (
        a: modport InterfaceA::x,
    ) {
        assign a.a = 1;
    }

    interface InterfaceA {
        var a: logic;

        modport x {
            a: input,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidAssignment { .. }));
}

#[test]
fn invalid_direction() {
    let code = r#"
    entity EntityA (
        a: ref logic,
    ) {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidDirection { .. }));

    let code = r#"
    entity EntityB (
        b: import logic,
    ) {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidDirection { .. }));

    let code = r#"
    entity EntityC {
        function FuncC (
            c: import logic,
        ) {}
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidDirection { .. }));

    let code = r#"
    entity EntityD {
        function FuncD (
            D: modport logic,
        ) {}
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidDirection { .. }));

    let code = r#"
    interface InterfaceE {
        var e: logic;
        modport mp {
            e: ref,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidDirection { .. }));

    let code = r#"
    interface InterfaceF {
        var f: logic;
        modport mp {
            f: modport,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidDirection { .. }));
}

#[test]
fn invalid_import() {
    let code = r#"
    entity EntityA {
        var a: logic;
        import a::*;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidImport { .. }));
}

#[test]
fn invalid_lsb() {
    let code = r#"
    entity EntityA {
        var a: logic;
        assign a = lsb;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidLsb { .. }));
}

#[test]
fn invalid_msb() {
    let code = r#"
    entity EntityA {
        var a: logic;
        assign a = msb;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidMsb { .. }));
}

#[test]
fn invalid_number_character() {
    let code = r#"
    entity EntityA {
        let a: logic = 1'b3;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidNumberCharacter { .. }
    ));
}

#[test]
fn invalid_statement() {
    let code = r#"
    entity EntityA (
        pwr: input logic,
        en: input logic,
    ) {
        sequence (pwr, en) {
            if_enable {
                if_enable {
                }
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidStatement { .. }));
}

#[test]
fn invalid_modport_item() {
    let code = r#"
    interface InterfaceA {
        var a: logic;
        function f -> logic {
            return 1;
        }

        modport mp {
            a: input ,
            f: import,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    interface InterfaceB {
        var a: logic;
        function f -> logic {
            return 1;
        }

        modport mp {
            f: input,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidModportVariableItem { .. }
    ));

    let code = r#"
    interface InterfaceC {
        var a: logic;
        function f -> logic {
            return 1;
        }

        modport mp {
            a: import,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidModportFunctionItem { .. }
    ));
}

#[test]
fn invalid_port_default_value() {
    let code = r#"
    entity EntityA (
        a: output logic = 0,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidPortDefaultValue { .. }
    ));

    let code = r#"
    entity EntityA (
        a: inout tri logic = 0,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidPortDefaultValue { .. }
    ));

    let code = r#"
    entity EntityA (
        a: ref logic = 0,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidPortDefaultValue { .. }
    ));

    let code = r#"
    entity EntityA {
        function FuncA(
            a: output logic = _,
        ) {
            a = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidPortDefaultValue { .. }
    ));
}

#[test]
fn mismatch_function_arity() {
    let code = r#"
    entity EntityA {
        function FuncA (
            a: input logic,
        ) -> logic {}

        let _a: logic = FuncA(1, 2);
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchFunctionArity { .. }
    ));

    let code = r#"
    interface InterfaceB {
        function FuncB (
            a: input logic,
        ) -> logic {}
    }

    entity EntityB {
        inst instB: InterfaceB();
        let _b: logic = instB.FuncB(1, 2);
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchFunctionArity { .. }
    ));

    let code = r#"
    interface InterfaceC {
        function FuncC (
            a: input logic,
        ) -> logic {}

        modport mp {
            FuncC: import,
        }
    }

    entity EntityC (
        ifc: modport InterfaceC::mp,
    ) {
        let _c: logic = ifc.FuncC(1, 2);
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchFunctionArity { .. }
    ));
}

#[test]
fn missing_default_generic_argument() {
    let code = r#"
    entity EntityA {
        function FuncA::<A: const> () -> logic<A> {}
        let _a: logic = FuncA::<1>();

        function FuncB::<A: const, B: const, C: const> () -> logic<A + B + C> {}
        let _b: logic = FuncB::<1, 2, 3>();

        function FuncC::<A: const = 1> () -> logic<A> {}
        let _c: logic = FuncC::<>();

        function FuncD::<A: const = 1, B: const = 2, C: const = 3> () -> logic<A + B + C> {}
        let _d: logic = FuncD::<>();

        function FuncE::<A: const, B: const = 2, C: const = 3> () -> logic<A + B + C> {}
        let _e: logic = FuncE::<1>();

        function FuncF::<A: const, B: const, C: const = 3> () -> logic<A + B + C> {}
        let _f: logic = FuncF::<1, 2>();
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
        entity EntityB {
            function FuncA::<A: const = 1, B: const, C: const = 3> () -> logic<A + B + C> {}
            let _a: logic = FuncA::<1, 2, 3> ();
        }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingDefaultArgument { .. }
    ));

    let code = r#"
        entity EntityC {
            function FuncA::<A: const = 1, B: const = 2, C: const> () -> logic<A + B + C> {}
            let _a: logic = FuncA::<1, 2, 3>();
        }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingDefaultArgument { .. }
    ));
}

#[test]
fn mismatch_generics_arity() {
    let code = r#"
    entity EntityA {
        function FuncA::<T: const> (
            a: input logic<T>,
        ) -> logic<T> {}

        let _a: logic = FuncA::<1, 2>(1);
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchGenericsArity { .. }
    ));

    let code = r#"
    entity EntityB {
        function FuncA::<T: const, U: const> (
            a: input logic<T>,
        ) -> logic<T> {}

        let _a: logic = FuncA::<1>(1);
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchGenericsArity { .. }
    ));

    let code = r#"
    package PackageC::<W: const> {
        struct StructC {
            c: logic<W>,
        }
    }
    entity EntityC {
        var c: PackageC::<2>::StructC;
        assign c.c = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    package PackageD {
        function FuncD::<W: const> -> logic<W> {
            return 0;
        }
    }
    entity SubD::<W: const> {
        let _d: logic<W> = PackageD::FuncD::<W>();
    }
    entity TopD {
        inst u_subd_1: SubD::<1>();
        inst u_subd_2: SubD::<2>();
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn mismatch_attribute_args() {
    let code = r#"
    entity EntityA {
        #[sv]
        const a: u32 = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchAttributeArgs { .. }
    ));
}

#[test]
fn mismatch_type() {
    let code = r#"
    entity EntityA {
        const a: u32 = 1;
        inst u: a;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityB_0 {}
    entity EntityB_1 {
        inst u: EntityB_0;
        let _b: u = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityC {
        function FuncC() -> logic {
            return 0;
        }
        let _c: FuncC = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityD_0 {}
    entity EntityD_1 {
        let _d: EntityD_0 = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceE {}
    entity EntityE {
        let _e: InterfaceE = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    package PackageF {}
    entity EntityF {
        let _f: PackageF = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityG {
        function FuncG::<T: type> -> T {
            var g: T;
            g = 0;
            return g;
        }

        let _g: logic = FuncG::<2>();
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityH {
        function FuncH::<T: type> -> T {
            var h: T;
            h = 0;
            return h;
        }

        type my_logic = logic;
        let _h: logic = FuncH::<my_logic>();
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    interface InterfaceI {}

    entity EntityI (
        a: modport InterfaceI,
    ) {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityJ {
        function FuncJ::<T: type> -> T {
            var g: T;
            g = 0;
            return g;
        }

        const X: u32 = 1;
        let _g: logic = FuncJ::<X>();
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    proto entity ProtoK0;
    proto entity ProtoK1;

    entity EntityK0::<T: ProtoK0> {
        inst u: T;
    }

    entity EntityK1 for ProtoK1 {}

    entity EntityK2 {
        inst u: EntityK0::<EntityK1>();
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceL0 {}
    interface InterfaceL1 {
      inst u: InterfaceL0();
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityM0 {}
    interface InterfaceM1 {
      inst u: EntityM0();
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceN1 {
        var a: logic;
        modport mp {
            a: input,
        }
    }
    entity EntityN1 (
        port_n1: input InterfaceN1::mp,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceN2::<W: const> {
        var a: logic<W>;
        modport mp {
            a: input,
        }
    }
    entity EntityN2 (
        port_n2: input InterfaceN2::<2>::mp,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceN3 {
        var a: logic;
    }
    entity EntityN3 (
        port_n3: input InterfaceN3,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceN4::<W: const> {
        var a: logic<W>;
    }
    entity EntityN4 (
        port_n4: input InterfaceN4::<2>,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceO1 {
        var a: logic;
    }
    entity EntityO1 (
        port_o1: modport InterfaceO1,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    interface InterfaceO2::<W: const> {
        var a: logic<W>;
    }
    entity EntityO2 (
        port_o2: modport InterfaceO2::<2>,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityA {
        enum EnumA {
            B,
        }
        let _a: EnumA = EnumA::B;
        let _b: EnumA = _a::B;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));

    let code = r#"
    entity EntityA {
        enum EnumA {
            A,
            B,
        }
        type EnumB = EnumA;

        let _a: EnumA = EnumA::A;
        let _b: EnumB = EnumB::B;
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn missing_if_enable() {
    let code = r#"
    entity EntityA (
        pwr: input logic,
        en: input logic,
    ) {
        sequence(pwr, en) {
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MissingIfReset { .. }));
}

#[test]
fn missing_port() {
    let code = r#"
    entity EntityA {
        inst u: EntityB;
    }

    entity EntityB (
        pwr: input logic,
    ) {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MissingPort { .. }));

    let code = r#"
    entity EntityA {
        inst u: EntityB;
    }

    entity EntityB (
        i_a: input  logic = 0,
        o_b: output logic = _,
    ) {
        assign o_b = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn missing_power_signal() {
    let code = r#"
    entity EntityA (
        pwr: input power
    ){
        sequence {}
        sequence (pwr) {}
        for i in 0..1 : g {
            sequence {}
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityB (
        pwr: input logic
    ){
        sequence {}
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingPowerSignal { .. }
    ));

    let code = r#"
    entity EntityC (
        pwr_0: input `_0 power,
        pwr_1: input `_1 power
    ){
        sequence {}
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingPowerSignal { .. }
    ));

    let code = r#"
    entity EntityD (
        pwr: input power<2>
    ){
        sequence {}
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingPowerSignal { .. }
    ));

    let code = r#"
    entity EntityE (){
        for i in 0..1 : g {
            let _pwr: power = 0;
            sequence {}
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingPowerSignal { .. }
    ));
}

#[test]
fn missing_enable_signal() {
    let code = r#"
    entity EntityA (
        pwr: input power,
        en: input enable
    ) {
        sequence {
            if_enable {}
        }
        sequence (pwr) {
            if_enable {}
        }
        sequence (pwr, en) {
            if_enable {}
        }
        for i in 0..1 : g {
            sequence {
                if_enable {}
            }
            sequence (pwr) {
                if_enable {}
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityB (
        pwr: input power,
        en: input logic
    ) {
        sequence {
            if_enable {}
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingResetSignal { .. }
    ));

    let code = r#"
    entity EntityC (
        pwr: input power,
        en: input logic
    ) {
        sequence (pwr) {
            if_enable {}
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingResetSignal { .. }
    ));

    let code = r#"
    entity EntityD (
        pwr:   input power,
        rst_0: input enable,
        rst_1: input enable
    ) {
        sequence {
            if_enable {}
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingResetSignal { .. }
    ));

    let code = r#"
    entity EntityE (
        pwr: input power,
        en: input enable<2>
    ) {
        sequence {
            if_enable {}
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingResetSignal { .. }
    ));

    let code = r#"
    entity EntityF (
        pwr: input power
    ) {
        for i in 0..1 : g {
            let _rst: enable = 0;
            sequence {
                if_enable {}
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingResetSignal { .. }
    ));
}

#[test]
fn missing_enable_statement() {
    let code = r#"
    entity EntityA (
        pwr: input power,
        en: input enable,
    ) {
        var a: logic;

        sequence(pwr, en) {
            if_enable {
            } else {
                a = 1;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingResetStatement { .. }
    ));
}

#[test]
fn missing_tri() {
    let code = r#"
    entity EntityA (
        x: inout logic,
    ) {
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MissingTri { .. }));
}

#[test]
fn missing_power_domain() {
    let code = r#"
    entity EntityA (
        pwr0: input power,
        pwr1: input power,
    ) {
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MissingPowerDomain { .. }
    ));
}

#[test]
fn too_large_enum_variant() {
    let code = r#"
    entity EntityA {
        enum EnumA: logic<2> {
            A = 100,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::TooLargeEnumVariant { .. }
    ));

    let code = r#"
    entity EntityB {
        enum EnumB: logic<2> {
            A = 3,
            B,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::TooLargeEnumVariant { .. }
    ));
}

#[test]
fn too_large_number() {
    let code = r#"
    entity EntityA {
        const a: u32 = 2'd100;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::TooLargeNumber { .. }));
}

#[test]
fn too_much_enum_variant() {
    let code = r#"
    entity EntityA {
        enum EnumA: logic<2> {
            A,
            B,
            C,
            D,
            E,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::TooMuchEnumVariant { .. }
    ));

    let code = r#"
    entity EntityB {
        enum EnumB: logic {
            A,
            B,
            C,
            D,
            E,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::TooMuchEnumVariant { .. }
    ));
}

#[test]
fn unevaluatable_enum_variant() {
    let code = r#"
    entity EntityA {
        enum EnumA: logic<2> {
            A = 2'b0x,
            B = 2'b10,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityB {
        enum EnumA: logic<2> {
            A = 2'b0x,
            B,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::UnevaluatableEnumVariant { .. }
    ));

    let code = r#"
    entity EntityC {
        #[enum_encoding(onehot)]
        enum EnumA: logic<2> {
            A = 2'bx1,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::UnevaluatableEnumVariant { .. }
    ));

    let code = r#"
    entity EntityD {
        #[enum_encoding(gray)]
        enum EnumA: logic<2> {
            A = 2'bx0,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::UnevaluatableEnumVariant { .. }
    ));
}

#[test]
fn invalid_enum_variant() {
    let code = r#"
    entity EntityA {
        #[enum_encoding(onehot)]
        enum EnumA{
            A = 0,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidEnumVariant { .. }
    ));

    let code = r#"
    entity EntityB {
        #[enum_encoding(onehot)]
        enum EnumA{
            A = 3,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidEnumVariant { .. }
    ));

    let code = r#"
    entity EntityC {
        #[enum_encoding(gray)]
        enum EnumA{
            A,
            B = 3,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidEnumVariant { .. }
    ));
}

#[test]
fn undefined_identifier() {
    let code = r#"
    entity EntityA {
        assign a = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::UndefinedIdentifier { .. }
    ));
}

#[test]
fn referring_package_before_definition() {
    let code = r#"
    entity EntityA {
        const A: u32 = PakcageB::B;
    }
    package PakcageB {
        const B: u32 = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::ReferringPackageBeforeDefinition { .. }
    ));

    let code = r#"
    interface InterfaceA {
        const A: u32 = PakcageB::B;
    }
    package PakcageB {
        const B: u32 = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::ReferringPackageBeforeDefinition { .. }
    ));

    let code = r#"
    package PackageA {
        const A: u32 = PakcageB::B;
    }
    package PakcageB {
        const B: u32 = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::ReferringPackageBeforeDefinition { .. }
    ));

    let code = r#"
    import PakcageB::B;
    package PakcageB {
        const B: u32 = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::ReferringPackageBeforeDefinition { .. }
    ));
}

#[test]
fn unknown_attribute() {
    let code = r#"
    entity EntityA {
        #[dummy_name]
        const a: u32 = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownAttribute { .. }));
}

#[test]
fn unknown_embed_lang() {
    let code = r#"
    embed (inline) x{{{
    }}}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownEmbedLang { .. }));
}

#[test]
fn unknown_embed_way() {
    let code = r#"
    embed (x) sv{{{
    }}}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownEmbedWay { .. }));
}

#[test]
fn unknown_member() {
    let code = r#"
    entity EntityA {
        struct StructA {
            memberA: logic,
        }
        var a: StructA;
        assign a.memberB = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownMember { .. }));

    let code = r#"
    entity EntityA (
        a_if: interface
    ) {
        assign a_if.a = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityA (
        a_if: interface::mp
    ) {
        assign a_if.a = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn unknown_msb() {
    let code = r#"
    entity EntityA {
        var a: $sv::SvType;
        let b: logic = a[msb];
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownMsb { .. }));

    let code = r#"
    interface InterfaceA {
        var a: logic<2>;
        modport mp {
            a: input
        }
    }
    entity EntityA (
        if_a: modport InterfaceA::mp
    ) {
        let a: logic = if_a.a[msb];
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownMsb { .. }));

    let code = r#"
    interface InterfaceA #(
        param W: u32 = 2
    ){
        var a: logic<W>;
    }
    entity EntityA {
        inst if_a: InterfaceA;
        assign if_a.a = 0;
        let a: logic = if_a.a[msb];
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownMsb { .. }));

    let code = r#"
    package PackageA::<W: const> {
        struct StructA {
            a: logic<W>,
        }
    }
    package PackageB {
        const B: u32 = 2;
    }
    package PackageC {
        const C: bit<2> = 0;
    }
    entity EntityA {
        var a: PackageA::<PackageB::B>::StructA;
        assign a.a = 0;
        let _b: logic = a.a[msb];
        let _c: logic = PackageC::C[msb];
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn uknown_port() {
    let code = r#"
    entity EntityA (
        pwr: input logic,
    ) {
        inst u: EntityB (
            pwr
        );
    }

    entity EntityB {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownPort { .. }));
}

#[test]
fn uknown_param() {
    let code = r#"
    entity EntityA {
        inst u: EntityB #(
            X: 1,
        )();
    }

    entity EntityB {}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownParam { .. }));
}

#[test]
fn unused_variable() {
    let code = r#"
    entity EntityA {
        let a: logic = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnusedVariable { .. }));

    let code = r#"
    entity EntityB {
        always_comb {
            let a: logic = 1;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnusedVariable { .. }));
}

#[test]
fn unused_return() {
    let code = r#"
    entity EntityA {
        function FuncA () -> logic {
            return 1;
        }

        initial {
            FuncA();
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnusedReturn { .. }));

    let code = r#"
    interface InterfaceB {
        function FuncB () -> logic {
            return 1;
        }
    }
    entity EntityB {
        inst ifb: InterfaceB ();
        initial {
            ifb.FuncB();
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnusedReturn { .. }));

    let code = r#"
    interface InterfaceC {
        modport mp {
            FuncC: import,
        }
        function FuncC() -> logic {
            return 1;
        }
    }
    entity EntityC (
        ifc: modport InterfaceC::mp,
    ){
        initial {
            ifc.FuncC();
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnusedReturn { .. }));
}

#[test]
fn break_outside_loop() {
    let code = r#"
    entity EntityA {
        always_comb {
            break;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidStatement { .. }));

    let code = r#"
    entity EntityA {
        always_comb {
            if 1 {
                break;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidStatement { .. }));
}

#[test]
fn unassign_variable() {
    let code = r#"
    entity EntityA {
        var _a: logic;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnassignVariable { .. }));

    let code = r#"
    entity EntityA {
        var a: logic;
        var b: logic;
        always_comb {
            b = a;
            a = 1;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnassignVariable { .. }));

    let code = r#"
    entity EntityA {
        var a: logic;
        always_comb {
            a = a;
            a = 1;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnassignVariable { .. }));

    let code = r#"
    entity EntityA {
        var a: logic;
        var b: logic;
        always_comb {
            if 1 {
                let c: logic = 1;
                a = c;
            } else {
                a = 0;
            }
            if 1 {
                var c: logic;
                b = c;
            } else {
                b = 0;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnassignVariable { .. }));

    let code = r#"
    entity EntityA {
        var a: logic;
        always_comb {
            let b: logic = 1;
            a = b;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityA {
        var a: logic;
        always_comb {
            for i: u32 in 0..1 {
                a = i;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityA {
        var a: logic<2>;
        always_comb {
            a[0] = 0;
        }

        always_comb {
            a[1] = a[0];
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityA {
        var a:  logic;
        var b:  logic;

        always_comb {
            a = b;
        }

        assign b = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityA (
        o_d:    output logic
    ) {
        assign  o_d = '0;
    }
    entity EntityB {
        var a: logic;
        var b: logic;

        always_comb {
            a = b;
        }

        inst u_sub: EntityA (
            o_d: b
        );
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    interface InterfaceA {
        function FuncA(
            a: output logic,
            b: ref    logic
        ) {
            a = 0;
            b = 1;
        }
        modport mp {
            FuncA: import,
        }
    }
    entity EntityA (
        if_a: modport InterfaceA::mp
    ){
        function FuncB(
            a: output logic,
            b: ref    logic,
        ) -> logic {
            a = 0;
            b = 1;
            return 0;
        }

        var _a: logic;
        var _b: logic;
        var _c: logic;
        var _d: logic;
        var _e: logic;

        always_comb {
            if_a.FuncA(_a, _b);
        }

        always_comb {
            _c = FuncB(_d, _e);
        }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn uncovered_branch() {
    let code = r#"
    entity EntityA {
        var a: logic;
        var b: logic;
        let x: logic = 1;

      always_comb {
        if x {
            let y: logic = 1;
            a = y;
        } else {
            a = 0;
        }
      }

      always_comb {
        var z: logic;
        if x {
            z = 1;
            b = z;
        } else {
            b = 0;
        }
      }
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityB {
        var a: logic;
        let x: logic = 1;

        always_comb {
            if x {
                a = 1;
            }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UncoveredBranch { .. }));
}

#[test]
fn anonymous_identifier() {
    let code = r#"
    entity EntityA (
        i_pwr: input `_ power,
    ) {
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity _ {
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::AnonymousIdentifierUsage { .. }
    ));

    let code = r#"
    entity EntityA {
        let a: logic = _ + 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::AnonymousIdentifierUsage { .. }
    ));

    let code = r#"
    entity SubA (
        i_a: input logic
    ) {
    }
    entity EntityA {
        inst u_sub: SubA (
            i_a: _
        );
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::AnonymousIdentifierUsage { .. }
    ));

    let code = r#"
    entity SubA (
        o_a: output logic
    ) {
        assign o_a = 0;
    }
    entity EntityA {
        inst u_sub: SubA (
            o_a: _
        );
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityA {
        inst u_sub: $sv::Sub (
            i_a: _,
            o_b: _,
        );
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityA (
        i_a: input logic = _,
    ) {}
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::AnonymousIdentifierUsage { .. }
    ));

    let code = r#"
    entity EntityA (
        o_a: output logic = _,
    ) {
        assign o_a = 0;
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn reserved_identifier() {
    let code = r#"
    entity __EntityA {
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::ReservedIdentifier { .. }
    ));
}

#[test]
fn enable_value_non_elaborative() {
    let code = r#"
    entity EntityA (
        i_pwr: input power,
        i_rst: input enable,
    ) {
        var a: logic;
        var b: logic;

        sequence {
            if_enable {
                a = b;
            } else {
                a = 1'b0;
            }
        }
    }"#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidResetNonElaborative { .. }
    ));
}

#[test]
fn invalid_factor_kind() {
    let code = r#"
    entity EntityA {
        function f (
            a: input logic,
        ) -> logic {
            return a;
        }

        var a: logic;

        assign a = f + 1;
    }"#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidFactor { .. }));

    let code = r#"
    interface InterfaceA {
        var a: logic;
        modport master {
            a: input,
        }
    }
    entity EntityA (
        b: modport InterfaceA::master,
    ) {
        var a: logic;
        always_comb {
            a = b;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidFactor { .. }));

    let code = r#"
    entity EntityA #(
        param A: bit = 0
    )(
        a: input logic = A,
    ){}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidFactor { .. }));

    let code = r#"
    package PackageA {
        const A: bit = 0;
    }
    entity EntityA (
        a: input  logic = PackageA::A,
    ){}
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn call_non_function() {
    let code = r#"
    entity EntityA {
        function f (
            a: input logic,
        ) -> logic {
            return a;
        }

        var a: logic;
        var b: logic;

        assign a = b() + 1;
    }"#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::CallNonFunction { .. }));
}

#[test]
fn invalid_assignment_to_const() {
    let code = r#"
    entity EntityA (
        i_pwr: input power,
        i_rst: input enable,
    ) {
        always_comb {
            let y: logic = 1;
            y = 0;
        }
    }"#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidAssignmentToConst { .. }
    ));
}

// TODO disable until adding expression type check
//#[test]
//fn test_factors() {
//    let code = r#"
//    interface InterfaceA {
//        var a: logic;
//        modport master {
//            a: input,
//        }
//    }
//
//    entity EntityA #(
//        param K: i32 = 32,
//    ) (
//        i_pwr: input   power             ,
//        i_rst: input   enable             ,
//        mst  : modport InterfaceA::master,
//    ) {
//
//        enum State: logic<3> {
//            Idle = 3'bxx1,
//            Run0 = 3'b000,
//            Run1 = 3'b010,
//            Run2 = 3'b100,
//            Done = 3'b110,
//        }
//
//        struct S {
//            v: logic,
//        }
//
//        union U {
//            v: logic,
//            w: logic,
//        }
//
//        let state: State = State::Run1;
//        var u    : U    ;
//        var s    : S    ;
//        const J    : i32   = 32;
//
//        for i in 0..1 :g_display {
//            sequence {
//                $display("%d", i);
//            }
//        }
//
//        function foo () -> logic {
//            return 1'b1;
//        }
//
//        function bar (
//            l: input logic,
//        ) -> logic {
//            return foo();
//        }
//
//        assign u.v = 1'b1;
//        assign s.v = 1'b1;
//        initial {
//            $display("%d", u);
//            $display("%d", s);
//            $display("%d", state);
//            $display("%d", mst.a);
//            $display("%d", i_pwr);
//            $display("%d", K);
//            $display("%d", J);
//            $display("%d", foo());
//            // Using $bits as a placeholder SystemFunciton.
//            $display("%d", $bits(S));
//            $display("%d", $bits(U));
//            $display("%d", $bits(foo(), State));
//            $display("%d", bar($bits(State)));
//            $display("%d", $bits(State));
//            // The following 4 cases should be error.
//            $display("%d", bar(S));
//            $display("%d", bar(U));
//            $display("%d", bar(State));
//            $display("%d", $bits(bar(State)));
//        }
//    }"#;
//
//    let errors = analyze(code);
//    assert!(errors.len() == 4);
//    for error in errors {
//        assert!(matches!(error, AnalyzerError::InvalidFactor { .. }));
//    }
//}

#[test]
fn enum_non_const_exception() {
    let code = r#"
    entity EntityA (
        i_pwr: input power,
        i_rst: input enable,
    ) {

        enum State: logic<3> {
            Idle = 3'bxx1,
            Run0 = 3'b000,
            Run1 = 3'b010,
            Run2 = 3'b100,
            Done = 3'b110,
        }
        var state: State;

        sequence {
            if_enable {
                state = State::Idle;
            }
        }

    }"#;
    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn invalid_case_condition_expression() {
    let code = r#"
    entity EntityA (
        i_sel: input  logic<3>,
        i_a  : input  logic<4>,
        o_b  : output logic,
        o_c  : output logic,
    ) {
        const ONE: bit <3> = 3'd1;

        always_comb {
          case i_sel {
            3'd0   : o_b = i_a[0];
            ONE    : o_b = i_a[1];
            2..=3  : o_b = i_a[2];
            3'b1xx : o_b = i_a[3];
            default: o_b = i_a[3];
          }
        }

        assign o_c = case i_sel {
            3'd0   : i_a[0],
            ONE    : i_a[1],
            2..=3  : i_a[2],
            3'b1xx : i_a[3],
            default: i_a[3],
        };
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());

    let code = r#"
    entity EntityB (
        i_sel: input  logic<2>,
        i_a  : input  logic<3>,
        o_b  : output logic,
    ) {
        let c: logic<2> = 2'd0;

        always_comb {
          case i_sel {
            c      : o_b = i_a[0];
            default: o_b = i_a[1];
          }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidCaseConditionNonElaborative { .. }
    ));

    let code = r#"
    entity EntityC (
        i_sel: input  logic<2>,
        i_a  : input  logic<3>,
        o_b  : output logic,
    ) {
        let c: logic<2> = 2'd1;

        always_comb {
          case i_sel {
            0..=c  : o_b = i_a[0];
            default: o_b = i_a[1];
          }
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidCaseConditionNonElaborative { .. }
    ));

    let code = r#"
    entity EntityD (
        i_sel: input  logic<2>,
        i_a  : input  logic<3>,
        o_b  : output logic,
    ) {
        let c: logic<2> = 2'd0;

        assign o_b = case i_sel {
            c      : i_a[0],
            default: i_a[1],
        };
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidCaseConditionNonElaborative { .. }
    ));

    let code = r#"
    entity EntityE (
        i_sel: input  logic<2>,
        i_a  : input  logic<3>,
        o_b  : output logic,
    ) {
        let c: logic<2> = 2'd1;

        assign o_b = case i_sel {
            0..=c  : i_a[0],
            default: i_a[1],
        };
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::InvalidCaseConditionNonElaborative { .. }
    ));
}

#[test]
fn invalid_cast() {
    let code = r#"
    entity EntityA {
        let a: power = 1;
        let _b: enable = a as enable;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidCast { .. }));
}

#[test]
fn invalid_test() {
    let code = r#"
    entity EntityA {}

    #[test(TestA)]
    embed (cocotb) py {{{
    }}}
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::InvalidTest { .. }));
}

#[test]
fn power_domain() {
    let code = r#"
    entity EntityA (
        i_pwr: input  `a power,
        i_dat: input  `a logic,
        o_dat: output `b logic,
    ) {
        assign o_dat = i_dat;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchPowerDomain { .. }
    ));

    let code = r#"
    entity EntityB (
        i_pwr : input  `a power,
        i_dat0: input  `a logic,
        i_dat1: input  `b logic,
        o_dat : output `a logic,
    ) {
        assign o_dat = {i_dat0, i_dat1};
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchPowerDomain { .. }
    ));

    let code = r#"
    entity EntityC (
        i_pwr : input  `a power,
        i_dat0: input  `a logic,
        i_dat1: input  `b logic,
        o_dat : output `a logic,
    ) {
        inst u: EntityD (
            i_dat: i_dat1,
            o_dat,
        );
    }

    entity EntityD (
        i_dat: input  logic,
        o_dat: output logic,
    ) {
        assign o_dat = i_dat;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchPowerDomain { .. }
    ));

    let code = r#"
    entity EntityE (
        i_pwr : input  `a power,
        i_dat0: input  `a logic,
        i_dat1: input  `b logic,
        o_dat : output `a logic,
    ) {
        inst u: $sv::Entity (
            i_dat: i_dat1,
            o_dat,
        );
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchPowerDomain { .. }
    ));

    let code = r#"
    entity EntityF (
        i_pwr : input  `a power,
        i_dat0: input  `a logic,
        i_dat1: input  `b logic,
        o_dat : output `b logic,
    ) {
        var r_dat: `b logic;

        sequence {
            r_dat = i_dat1;
        }

        assign o_dat = r_dat;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchPowerDomain { .. }
    ));

    let code = r#"
    entity EntityG (
        i_pwr: input   `a power,
        i_dat: input   `a logic,
        o_dat: modport `b InterfaceA::port,
    ) {
        assign o_dat.a = i_dat;
    }

    interface InterfaceA {
        var a: logic;

        modport port {
            a: output,
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchPowerDomain { .. }
    ));

    let code = r#"
    entity EntityH (
        i_pwr: input  `a power,
        i_dat: input  `a logic,
        o_dat: output `b logic,
    ) {
        always_comb {
            o_dat = i_dat;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::MismatchPowerDomain { .. }
    ));
}

#[test]
fn r#unsafe() {
    let code = r#"
    entity EntityA {
        unsafe(x) {
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::UnknownUnsafe { .. }));
}

#[test]
fn detect_recursive() {
    let code = r#"
    entity EntityA (
        x: input x,
    ) {
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::MismatchType { .. }));
}

#[test]
fn sv_keyword_usage() {
    let code = r#"
    entity EntityA {
        var always: logic;
        assign always = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::SvKeywordUsage { .. }));

    let code = r#"
    entity EntityA {
        var r#always: logic;
        assign r#always = 1;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::SvKeywordUsage { .. }));
}

#[test]
fn sv_with_implicit_enable() {
    let code = r#"
    entity EntityA {
        var en: enable;

        inst u: $sv::Entity (
            en,
        );
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::SvWithImplicitEnable { .. }
    ));

    let code = r#"
    entity EntityB {
        var en: enable_low;

        inst u: $sv::Entity (
            en,
        );
    }
    "#;

    let errors = analyze(code);
    assert!(errors.is_empty());
}

#[test]
fn conflict_with_mangled_enum_member() {
    let code = r#"
    entity EntityA {
        enum EnumA: logic {
            MemberA,
        }
        var EnumA_MemberA: logic;
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(
        errors[0],
        AnalyzerError::DuplicatedIdentifier { .. }
    ));
}

#[test]
fn unresolvable_generic_argument() {
    let code = r#"
    entity EntityA {
        const X: u32 = 1;
        const Y: u32 = PackageA::<X>::W;
    }

    package PackageA::<T: const> {
        const W: u32 = T;
    }
    "#;

    let errors = analyze(code);
    // This pattern also causes CyclicTypeDependency error
    assert!(errors
        .iter()
        .any(|e| matches!(e, AnalyzerError::UnresolvableGenericArgument { .. })));
}

#[test]
fn wrong_seperator() {
    let code = r#"
    package A {
        enum B {
            C,
        }
    }
    entity Entity {
        var _a: A::B;

        always_comb {
            _a = A.B.C;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::WrongSeparator { .. }));

    let code = r#"
    package A {
        enum B {
            C,
        }
    }
    entity Entity {
        var _a: A::B;

        always_comb {
            _a = A::B.C;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::WrongSeparator { .. }));

    let code = r#"
    entity Entity {
        struct B {
            b: logic,
        }

        var _a: B;
        always_comb {
            _a::b = '0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::WrongSeparator { .. }));

    let code = r#"
    interface B {
        var c: logic;
        modport mp {
            c: input,
        }
    }
    entity Entity (
        b: modport B::mp
    ) {
        var _a: logic;
        always_comb {
            _a = b::c;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::WrongSeparator { .. }));

    let code = r#"
    interface A {
        var b: logic;
    }
    entity Entity {
        inst a: A;
        always_comb {
            a::b = 0;
        }
    }
    "#;

    let errors = analyze(code);
    assert!(matches!(errors[0], AnalyzerError::WrongSeparator { .. }));
}
