use crate::core::core;
use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

lalrpop_mod!(pub grammar);

pub fn parse_program<'src>(
    s: &'src str,
) -> Result<core::Program, ParseError<usize, Token<'_>, &'static str>> {
    grammar::ProgramParser::new().parse(s)
}

static S1: &'static str = r#"
fn(f: fn[T](T) -> T, x: Int) => f[Int](x)
"#;

static S2: &'static str = r#"
letrec
    function fst[T, U](x: (T, U)) -> T
    begin
        x.0
    end
    function snd[T, U](x: (T, U)) -> U
    begin
        x.1
    end
in
    let x = ((42, 'a'), true) in
    snd(fst(x))
end
"#;

static S3: &'static str = r#"
let counter = pack (42, fn(x: Int) => x) as [T=Int](Int, fn(Int) -> Int) in
unpack counter2[T] = counter in
counter2.1(counter2.0)
"#;

static S4: &'static str = r#"
let x = ((), ()) in
let y = pack x as [T=()](T, T) in
x
"#;

#[test]
fn grammar_test() {
    assert_eq!(grammar::BoolParser::new().parse("false").unwrap(), false);
    assert_eq!(grammar::IntParser::new().parse("-42").unwrap(), -42);
    assert!(grammar::FloatParser::new().parse("-114.514").unwrap() - -114.514 < 0.00001);
    assert_eq!(grammar::CharParser::new().parse("'a'").unwrap(), 'a');
    assert_eq!(
        grammar::NameParser::new().parse("x").unwrap(),
        crate::common::name::Name::RawId(crate::common::intern::InternStr::new("x"))
    );

    assert!(grammar::ExprParser::new().parse(&S1).is_ok());
    assert!(grammar::ExprParser::new().parse(&S2).is_ok());
    assert!(grammar::ExprParser::new().parse(&S3).is_ok());
    assert!(grammar::ExprParser::new().parse(&S4).is_ok());
}
