use norem_lang::core;
use std::fmt::Write;
use std::{fs, io, path::Path};

pub fn test_file<S: AsRef<Path>>(path: S) -> Result<String, io::Error> {
    let src = fs::read_to_string(&path)?;
    let mut res = String::new();
    let prog = core::parse::parse_program(&src);
    let prog = match prog {
        Ok(prog) => {
            writeln!(&mut res, "{:#?}", prog).unwrap();
            prog
        }
        Err(err) => {
            writeln!(&mut res, "{}", err).unwrap();
            return Ok(res);
        }
    };

    let typ = core::check::check_prog(&prog);
    match typ {
        Ok(()) => writeln!(&mut res, "typecheck passed!").unwrap(),
        Err(err) => writeln!(&mut res, "typecheck failed! {:?}", err).unwrap(),
    };

    let val = core::eval::eval_prog(&prog);
    match val {
        Ok(val) => writeln!(&mut res, "evaluation result: {:?}", val).unwrap(),
        Err(err) => writeln!(&mut res, "evaluation failed! {:?}", err).unwrap(),
    }

    Ok(res)
}

#[test]
fn test_pair() {
    let actual = test_file(Path::new("./examples/core/Pair.core")).unwrap();
    let expect = expect_test::expect![[r#"
        Program {
            decls: [
                Decl {
                    name: RawId(
                        fst,
                    ),
                    gens: Some(
                        [
                            RawId(
                                T,
                            ),
                            RawId(
                                U,
                            ),
                        ],
                    ),
                    pars: [
                        (
                            RawId(
                                x,
                            ),
                            Tup {
                                flds: [
                                    Var {
                                        var: RawId(
                                            T,
                                        ),
                                    },
                                    Var {
                                        var: RawId(
                                            U,
                                        ),
                                    },
                                ],
                            },
                        ),
                    ],
                    res: Var {
                        var: RawId(
                            T,
                        ),
                    },
                    body: Sel {
                        expr: Var {
                            var: RawId(
                                x,
                            ),
                        },
                        idx: 0,
                    },
                },
                Decl {
                    name: RawId(
                        snd,
                    ),
                    gens: Some(
                        [
                            RawId(
                                T,
                            ),
                            RawId(
                                U,
                            ),
                        ],
                    ),
                    pars: [
                        (
                            RawId(
                                x,
                            ),
                            Tup {
                                flds: [
                                    Var {
                                        var: RawId(
                                            T,
                                        ),
                                    },
                                    Var {
                                        var: RawId(
                                            U,
                                        ),
                                    },
                                ],
                            },
                        ),
                    ],
                    res: Var {
                        var: RawId(
                            U,
                        ),
                    },
                    body: Sel {
                        expr: Var {
                            var: RawId(
                                x,
                            ),
                        },
                        idx: 1,
                    },
                },
                Decl {
                    name: RawId(
                        main,
                    ),
                    gens: None,
                    pars: [],
                    res: Lit {
                        lit: TyInt,
                    },
                    body: Let {
                        bind: RawId(
                            x,
                        ),
                        expr: Tup {
                            flds: [
                                Tup {
                                    flds: [
                                        Lit {
                                            lit: Char(
                                                'a',
                                            ),
                                        },
                                        Lit {
                                            lit: Int(
                                                42,
                                            ),
                                        },
                                    ],
                                },
                                Lit {
                                    lit: Bool(
                                        true,
                                    ),
                                },
                            ],
                        },
                        cont: App {
                            func: Inst {
                                expr: Var {
                                    var: RawId(
                                        snd,
                                    ),
                                },
                                typs: [
                                    Lit {
                                        lit: TyChar,
                                    },
                                    Lit {
                                        lit: TyInt,
                                    },
                                ],
                            },
                            args: [
                                App {
                                    func: Inst {
                                        expr: Var {
                                            var: RawId(
                                                fst,
                                            ),
                                        },
                                        typs: [
                                            Tup {
                                                flds: [
                                                    Lit {
                                                        lit: TyChar,
                                                    },
                                                    Lit {
                                                        lit: TyInt,
                                                    },
                                                ],
                                            },
                                            Lit {
                                                lit: TyBool,
                                            },
                                        ],
                                    },
                                    args: [
                                        Var {
                                            var: RawId(
                                                x,
                                            ),
                                        },
                                    ],
                                },
                            ],
                        },
                    },
                },
            ],
        }
        typecheck passed!
        evaluation result: Lit(Int(42))
    "#]];
    expect.assert_eq(&actual)
}

#[test]
fn test_rec_fib() {
    let actual = test_file(Path::new("./examples/core/RecFib.core")).unwrap();
    let expect = expect_test::expect![[r#"
        Program {
            decls: [
                Decl {
                    name: RawId(
                        fib,
                    ),
                    gens: None,
                    pars: [
                        (
                            RawId(
                                n,
                            ),
                            Lit {
                                lit: TyInt,
                            },
                        ),
                    ],
                    res: Lit {
                        lit: TyInt,
                    },
                    body: Ifte {
                        cond: Prim {
                            prim: ICmp(
                                Lt,
                            ),
                            args: [
                                Var {
                                    var: RawId(
                                        n,
                                    ),
                                },
                                Lit {
                                    lit: Int(
                                        1,
                                    ),
                                },
                            ],
                        },
                        trbr: Lit {
                            lit: Int(
                                0,
                            ),
                        },
                        flbr: Ifte {
                            cond: Prim {
                                prim: ICmp(
                                    Eq,
                                ),
                                args: [
                                    Var {
                                        var: RawId(
                                            n,
                                        ),
                                    },
                                    Lit {
                                        lit: Int(
                                            1,
                                        ),
                                    },
                                ],
                            },
                            trbr: Lit {
                                lit: Int(
                                    1,
                                ),
                            },
                            flbr: Prim {
                                prim: IAdd,
                                args: [
                                    App {
                                        func: Var {
                                            var: RawId(
                                                fib,
                                            ),
                                        },
                                        args: [
                                            Prim {
                                                prim: ISub,
                                                args: [
                                                    Var {
                                                        var: RawId(
                                                            n,
                                                        ),
                                                    },
                                                    Lit {
                                                        lit: Int(
                                                            1,
                                                        ),
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                    App {
                                        func: Var {
                                            var: RawId(
                                                fib,
                                            ),
                                        },
                                        args: [
                                            Prim {
                                                prim: ISub,
                                                args: [
                                                    Var {
                                                        var: RawId(
                                                            n,
                                                        ),
                                                    },
                                                    Lit {
                                                        lit: Int(
                                                            2,
                                                        ),
                                                    },
                                                ],
                                            },
                                        ],
                                    },
                                ],
                            },
                        },
                    },
                },
                Decl {
                    name: RawId(
                        main,
                    ),
                    gens: None,
                    pars: [],
                    res: Lit {
                        lit: TyInt,
                    },
                    body: App {
                        func: Var {
                            var: RawId(
                                fib,
                            ),
                        },
                        args: [
                            Lit {
                                lit: Int(
                                    10,
                                ),
                            },
                        ],
                    },
                },
            ],
        }
        typecheck passed!
        evaluation result: Lit(Int(55))
    "#]];
    expect.assert_eq(&actual)
}
