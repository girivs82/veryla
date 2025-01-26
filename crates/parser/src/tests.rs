use crate::Parser;

#[track_caller]
fn success(entitytype: &str, code: &str) {
    let code = format!("entity A of {} {{ {} }}", entitytype, code);
    let parser = Parser::parse(&code, &"");
    dbg!(code);
    assert!(parser.is_ok());
}

#[track_caller]
fn failure(entitytype: &str, code: &str) {
    let code = format!("entity A of {} {{ {} }}", entitytype, code);
    let parser = Parser::parse(&code, &"");
    dbg!(code);
    assert!(parser.is_err());
}

#[test]
fn entity_type() {
    let mut entitytype = "element".to_string();
    success(&entitytype, "");
    entitytype = entitytype.replace("element", "system");
    success(&entitytype, "");
    entitytype = entitytype.replace("system", "subsystem");
    success(&entitytype, "");
    entitytype = entitytype.replace("subsystem", "component");
    success(&entitytype, "");
    entitytype = entitytype.replace("component", "subcomponent");
    success(&entitytype, "");
    entitytype = entitytype.replace("subcomponent", "requirement");
    success(&entitytype, "");
    entitytype = entitytype.replace("requirement", "abcd");
    failure(&entitytype, "");
}

#[test]
fn comment() {
    success("element", "// aaaaa \n");
    success("element", "/* aaaaa */");
    success("element", "/* aa \n a \n aa */");
}

#[test]
fn number() {
    // integer
    success("element", "let a: u32 = 0123456789;");
    success("element", "let a: u32 = 0_1_23456789;");
    success("element", "let a: u32 = _0_1_23456789;"); // identifier
    failure("element", "let a: u32 = 0_1__23456789;");

    // binary
    success("element", "let a: u32 = 32'b01xzXZ;");
    success("element", "let a: u32 = 32'b01_xz_XZ;");
    failure("element", "let a: u32 = 32'b01__xz_XZ;");

    // octal
    success("element", "let a: u32 = 32'o01234567xzXZ;");
    success("element", "let a: u32 = 32'o01234567_xz_XZ;");
    failure("element", "let a: u32 = 32'o01234567__xz_XZ;");

    // decimal
    success("element", "let a: u32 = 32'd0123456789xzXZ;");
    success("element", "let a: u32 = 32'd0123456789_xz_XZ;");
    failure("element", "let a: u32 = 32'd0123456789__xz_XZ;");

    // hex
    success("element", "let a: u32 = 32'h0123456789abcdefABCDEFxzXZ;");
    success("element", "let a: u32 = 32'h0123456789abcdefABCDEF_xz_XZ;");
    failure("element", "let a: u32 = 32'h0123456789abcdefABCDEF__xz_XZ;");

    // all0, all1
    success("element", "let a: u32 = '0;");
    success("element", "let a: u32 = '1;");
    failure("element", "let a: u32 = '2;");

    // floating point
    success("element", "let a: u32 = 0.1;");
    success("element", "let a: u32 = 0_1_23.4_5_67;");
    failure("element", "let a: u32 = 0_1__23.4_5_67;");

    // exponent
    success("element", "let a: u32 = 0.1e10;");
    success("element", "let a: u32 = 0.1e+10;");
    success("element", "let a: u32 = 0.1e-10;");
    success("element", "let a: u32 = 0.1E+10;");
    success("element", "let a: u32 = 0.1E-10;");
    failure("element", "let a: u32 = 0.1e++10;");
    failure("element", "let a: u32 = 0.1e10.0;");
}

#[test]
fn delay() {
    success("element", "let a: u32 = #10 0.1e10;");
    success("element", "assign a = #10 1;");
    success("element", "always_comb { a = #20000 1; }");
    failure("element", "let a: u32 = #10.5 0.1e10;");
}

#[test]
fn identifier() {
    success("element", "var a: u32;");
    success("element", "var _abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_: u32;");
    failure("element", "var 0a: u32;");
}

#[test]
fn expression() {
    success("element", "let a: u32 = 1 && 1 || 1 & 1 ^ 1 ~^ 1 ^~ 1 | 1;");
    success("element", "let a: u32 = 1 <: 1 <= 1 >: 1 >= 1 == 1 != 1 === 1 !== 1 ==? 1 !=? 1;");
    success("element", "let a: u32 = 1 << 1 >> 1 <<< 1 >>> 1;");
    success("element", "let a: u32 = 1 ** 1 * 1 / 1 % 1 + 1 - 1;");
    success("element", "let a: u32 = +-!~&|^~&~|~^^~1;");
    success("element", "let a: u32 = ( (1 && 1) || 1) & (1 ^ 1 ~^ 1) ^~ 1 | 1;");
    failure("element", "let a: u32 = ( (1 && 1) || 1 & (1 ^ 1 ~^ 1) ^~ 1 | 1;");
}

#[test]
fn function_call() {
    success("element", "let a: u32 = a();");
    success("element", "let a: u32 = $a();");
    success("element", "let a: u32 = a.a.a();");
    success("element", "let a: u32 = a::a::a();");
    success("element", "let a: u32 = a::a::a.a.a();");
    success("element", "let a: u32 = a(1, 1, 1);");
    success("element", "let a: u32 = a(1, 1, 1,);");
    failure("element", "let a: u32 = a(1 1, 1,);");
}

#[test]
fn range() {
    success("element", "let a: u32 = a[1];");
    success("element", "let a: u32 = a[1:0];");
    success("element", "let a: u32 = a[1+:1];");
    success("element", "let a: u32 = a[1-:1];");
    success("element", "let a: u32 = a[1 step 1];");
}

#[test]
fn r#type() {
    success("element", "var a: logic;");
    success("element", "var a: analog;");
    success("element", "var a: bit;");
    success("element", "var a: u32;");
    success("element", "var a: u64;");
    success("element", "var a: i32;");
    success("element", "var a: i64;");
    success("element", "var a: f32;");
    success("element", "var a: f64;");
    success("element", "var a: a::a;");

    success("element", "var a: logic<10, 10>;");
    success("element", "var a: analog<10, 10>;");
    success("element", "var a: bit<10, 10>;");
    success("element", "var a: u32[10, 10];");
    success("element", "var a: u64[10, 10];");
    success("element", "var a: i32[10, 10];");
    success("element", "var a: i64[10, 10];");
    success("element", "var a: f32[10, 10];");
    success("element", "var a: f64[10, 10];");
    success("element", "var a: a::a<10, 10>;");
}

#[test]
fn assignment_statement() {
    success("element", "always_comb { a = 1; }");
    success("element", "always_comb { a.a.a = 1; }");
    success("element", "always_comb { a += 1; }");
    success("element", "always_comb { a -= 1; }");
    success("element", "always_comb { a *= 1; }");
    success("element", "always_comb { a /= 1; }");
    success("element", "always_comb { a %= 1; }");
    success("element", "always_comb { a &= 1; }");
    success("element", "always_comb { a |= 1; }");
    success("element", "always_comb { a ^= 1; }");
    success("element", "always_comb { a <<= 1; }");
    success("element", "always_comb { a >>= 1; }");
    success("element", "always_comb { a <<<= 1; }");
    success("element", "always_comb { a >>>= 1; }");
}
