use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

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
let counter = [T=Int](42, fn(x: Int) => x) in
unpack x[T] = counter in
x.1(x.0)
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

    let res = grammar::ExprParser::new().parse(&S1);
    println!("{:#?}", res);

    let res = grammar::ExprParser::new().parse(&S2);
    println!("{:#?}", res);

    let res = grammar::ExprParser::new().parse(&S3);
    println!("{:#?}", res);
}
