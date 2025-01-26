use crate::Emitter;
use std::path::PathBuf;
use veryla_analyzer::Analyzer;
use veryla_metadata::{PowerType, Metadata, EnableType};
use veryla_parser::Parser;

#[track_caller]
fn emit(metadata: &Metadata, code: &str) -> String {
    let parser = Parser::parse(&code, &"").unwrap();
    let analyzer = Analyzer::new(metadata);

    analyzer.analyze_pass1(&"prj", &code, &"", &parser.veryla);
    Analyzer::analyze_post_pass1();
    analyzer.analyze_pass2(&"prj", &code, &"", &parser.veryla);

    let mut emitter = Emitter::new(
        metadata,
        &PathBuf::from("test.veryla"),
        &PathBuf::from("test.sv"),
        &PathBuf::from("test.sv.map"),
    );
    emitter.emit(&"prj", &parser.veryla);
    emitter.as_str().to_string()
}

#[test]
fn prefix_suffix_power_posedge_enable_high() {
    let code = r#"module ModuleA (
    pwr: input power,
    en: input enable,
) {
    inst u: ModuleB (
        pwr,
        en,
    );

    let _a: logic = pwr;
    let _b: logic = en;

    var _c: logic;
    sequence {
        if_enable {
            _c = 0;
        } else {
            _c = 1;
        }
    }
}

module ModuleB (
    pwr: input power,
    en: input enable,
) {}
"#;

    let expect = r#"module prj_ModuleA (
    input logic pwr_pos_pwr_pwr_pos  ,
    input logic en_high_en_en_high
);
    prj_ModuleB u (
        .pwr_pos_pwr_pwr_pos   (pwr_pos_pwr_pwr_pos  ),
        .en_high_en_en_high (en_high_en_en_high)
    );

    logic _a;
    always_comb _a = pwr_pos_pwr_pwr_pos;
    logic _b;
    always_comb _b = en_high_en_en_high;

    logic _c;
    sequence @ (posedge pwr_pos_pwr_pwr_pos, posedge en_high_en_en_high) begin
        if (en_high_en_en_high) begin
            _c <= 0;
        end else begin
            _c <= 1;
        end
    end
endmodule

module prj_ModuleB (
    input logic pwr_pos_pwr_pwr_pos  ,
    input logic en_high_en_en_high
);
endmodule
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.power_type = PowerType::PosEdge;
    metadata.build.enable_type = EnableType::High;
    metadata.build.power_posedge_prefix = Some("pwr_pos_".to_string());
    metadata.build.power_posedge_suffix = Some("_pwr_pos".to_string());
    metadata.build.enable_high_prefix = Some("en_high_".to_string());
    metadata.build.enable_high_suffix = Some("_en_high".to_string());

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}

#[test]
fn prefix_suffix_power_negedge_enable_low() {
    let code = r#"module ModuleA (
    pwr: input power,
    en: input enable,
) {
    inst u: ModuleB (
        pwr,
        en,
    );

    let _a: logic = pwr;
    let _b: logic = en;

    var _c: logic;
    sequence {
        if_enable {
            _c = 0;
        } else {
            _c = 1;
        }
    }
}

module ModuleB (
    pwr: input power,
    en: input enable,
) {}
"#;

    let expect = r#"module prj_ModuleA (
    input logic pwr_neg_pwr_pwr_neg,
    input logic en_low_en_en_low
);
    prj_ModuleB u (
        .pwr_neg_pwr_pwr_neg (pwr_neg_pwr_pwr_neg),
        .en_low_en_en_low (en_low_en_en_low)
    );

    logic _a;
    always_comb _a = pwr_neg_pwr_pwr_neg;
    logic _b;
    always_comb _b = en_low_en_en_low;

    logic _c;
    sequence @ (negedge pwr_neg_pwr_pwr_neg) begin
        if (!en_low_en_en_low) begin
            _c <= 0;
        end else begin
            _c <= 1;
        end
    end
endmodule

module prj_ModuleB (
    input logic pwr_neg_pwr_pwr_neg,
    input logic en_low_en_en_low
);
endmodule
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.power_type = PowerType::NegEdge;
    metadata.build.enable_type = EnableType::Low;
    metadata.build.power_negedge_prefix = Some("pwr_neg_".to_string());
    metadata.build.power_negedge_suffix = Some("_pwr_neg".to_string());
    metadata.build.enable_low_prefix = Some("en_low_".to_string());
    metadata.build.enable_low_suffix = Some("_en_low".to_string());

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}

#[test]
fn omit_project_prefix() {
    let code = r#"module ModuleA {
}

package PackageA {
}

interface InterfaceA {
}
"#;

    let expect = r#"module ModuleA;
endmodule

package PackageA;
endpackage

interface InterfaceA;
endinterface
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.omit_project_prefix = true;

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}

#[test]
fn expand_case_statement() {
    let code = r#"module ModuleA {
    const y: bit = 1;

    var a: logic;
    let x: logic = 1;

    always_comb {
        case x {
            0: a = 1;
            1: a = 1;
            2: {
                   a = 1;
                   a = 1;
                   a = 1;
               }
            3, 4   : a = 1;
            5..=7  : a = 1;
            y - 1  : a = 1;
            default: a = 1;
        }
    }
}
"#;

    let expect = r#"module ModuleA;
    localparam bit y = 1;

    logic a;
    logic x;
    always_comb x = 1;

    always_comb begin
        case (1'b1)
            (x) ==? (0): a = 1;
            (x) ==? (1): a = 1;
            (x) ==? (2): begin
                             a = 1;
                             a = 1;
                             a = 1;
                         end
            (x) ==? (3), (x) ==? (4    ): a = 1;
            ((x) >= (5)) && ((x) <= (7)): a = 1;
            (x) ==? (y - 1             ): a = 1;
            default                     : a = 1;
        endcase
    end
endmodule
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.omit_project_prefix = true;
    metadata.build.expand_inside_operation = true;

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}

#[test]
fn expand_inside_operator() {
    let code = r#"module ModuleA {
    var a: logic;
    var b: logic;

    assign a = inside 1 + 2 / 3 {0, 0..10, 1..=10};
    assign b = outside 1 * 2 - 1 {0, 0..10, 1..=10};
    }
"#;

    let expect = r#"module ModuleA;
    logic a;
    logic b;

    always_comb a = ((1 + 2 / 3) ==? (0) || ((1 + 2 / 3) >= (0)) && ((1 + 2 / 3) < (10)) || ((1 + 2 / 3) >= (1)) && ((1 + 2 / 3) <= (10)));
    always_comb b = !((1 * 2 - 1) ==? (0) || ((1 * 2 - 1) >= (0)) && ((1 * 2 - 1) < (10)) || ((1 * 2 - 1) >= (1)) && ((1 * 2 - 1) <= (10)));
endmodule
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.omit_project_prefix = true;
    metadata.build.expand_inside_operation = true;

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}

#[test]
fn expand_case_expression() {
    let code = r#"module ModuleA {
    let a: logic = 1;
    var b: logic;

    assign b = case a {
        1      : 0,
        2      : 1,
        3, 4   : 2,
        5..=7  : 3,
        default: 4,
    };
}
"#;

    let expect = r#"module ModuleA;
    logic a;
    always_comb a = 1;
    logic b;

    always_comb b = (((a) ==? (1)) ? (
        0
    ) : ((a) ==? (2)) ? (
        1
    ) : ((a) ==? (3)) ? (
        2
    ) : ((a) ==? (4)) ? (
        2
    ) : (((a) >= (5)) && ((a) <= (7))) ? (
        3
    ) : (
        4
    ));
endmodule
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.omit_project_prefix = true;
    metadata.build.expand_inside_operation = true;

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}

#[test]
fn enable_cast() {
    let code = r#"module ModuleA {
    var a: enable_high;
    var b: enable_low;;

    let d: enable_high = a as enable_high;
    let e: enable_low  = a as enable_low ;
    let f: enable            = b as enable           ;
    let g: enable            = c as enable           ;
}
"#;

    let expect = r#"module prj_ModuleA;
    logic a;
    logic b;
    logic c;

    logic d;
    always_comb d = a;
    logic e;
    always_comb e = ~a;
    logic f;
    always_comb f = b;
    logic g;
    always_comb g = ~c;
endmodule
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.enable_type = EnableType::High;

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}


#[test]
fn emit_cond_type() {
    let code = r#"module ModuleA (
    i_pwr: input power,
    i_en: input enable,
) {
    let x: logic = 1;
    var a: logic;
    var b: logic;
    var c: logic;
    var d: logic;
    var e: logic;
    var f: logic;
    var g: logic;
    var h: logic;
    var i: logic;

    always_comb {
        #[cond_type(unique)]
        case x {
            0: a = 1;
            1: a = 1;
        }
        #[cond_type(unique0)]
        case x {
            0: b = 1;
            1: b = 1;
        }
        #[cond_type(priority)]
        case x {
            0: c = 1;
            1: c = 1;
        }
    }

    always_comb {
        #[cond_type(unique)]
        if x == 0 {
            d = 1;
        } else if x == 1 {
            d = 1;
        }
        #[cond_type(unique0)]
        if x == 0 {
            e = 1;
        } else if x == 1 {
            e = 1;
        }
        #[cond_type(priority)]
        if x == 0 {
            f = 1;
        } else if x == 1 {
            f = 1;
        }
    }

    sequence {
        #[cond_type(unique)]
        if_enable {
            g = 1;
        } else if x == 1 {
            g = 1;
        }
    }
    sequence {
        #[cond_type(unique0)]
        if_enable {
            h = 1;
        } else if x == 1 {
            h = 1;
        }
    }
    sequence {
        #[cond_type(priority)]
        if_enable {
            i = 1;
        } else if x == 1 {
            i = 1;
        }
    }
}
"#;

    let expect = r#"module prj_ModuleA (
    input logic i_pwr,
    input logic i_en
);
    logic x;
    always_comb x = 1;
    logic a;
    logic b;
    logic c;
    logic d;
    logic e;
    logic f;
    logic g;
    logic h;
    logic i;

    always_comb begin

        unique case (x) inside
            0: a = 1;
            1: a = 1;
        endcase

        unique0 case (x) inside
            0: b = 1;
            1: b = 1;
        endcase

        priority case (x) inside
            0: c = 1;
            1: c = 1;
        endcase
    end

    always_comb begin

        unique if (x == 0) begin
            d = 1;
        end else if (x == 1) begin
            d = 1;
        end

        unique0 if (x == 0) begin
            e = 1;
        end else if (x == 1) begin
            e = 1;
        end

        priority if (x == 0) begin
            f = 1;
        end else if (x == 1) begin
            f = 1;
        end
    end

    sequence @ (posedge i_pwr, negedge i_en) begin

        unique if (!i_en) begin
            g <= 1;
        end else if (x == 1) begin
            g <= 1;
        end
    end
    sequence @ (posedge i_pwr, negedge i_en) begin

        unique0 if (!i_en) begin
            h <= 1;
        end else if (x == 1) begin
            h <= 1;
        end
    end
    sequence @ (posedge i_pwr, negedge i_en) begin

        priority if (!i_en) begin
            i <= 1;
        end else if (x == 1) begin
            i <= 1;
        end
    end
endmodule
//# sourceMappingURL=test.sv.map
"#;

    let mut metadata: Metadata =
        toml::from_str(&Metadata::create_default_toml("prj").unwrap()).unwrap();

    metadata.build.emit_cond_type = true;

    let ret = if cfg!(windows) {
        emit(&metadata, code).replace("\r\n", "\n")
    } else {
        emit(&metadata, code)
    };

    assert_eq!(ret, expect);
}
