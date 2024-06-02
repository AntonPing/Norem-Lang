use norem_lang::core;
use std::fmt::Write;
use std::{fs, io, path::Path};

pub fn test_file<S: AsRef<Path>>(path: S) -> Result<String, io::Error> {
    let src = fs::read_to_string(&path)?;
    let mut res = String::new();
    let prog = core::parse::parse_program(&src);
    let _prog = match prog {
        Ok(prog) => {
            writeln!(&mut res, "{:#?}", prog).unwrap();
            prog
        }
        Err(err) => {
            writeln!(&mut res, "{}", err).unwrap();
            return Ok(res);
        }
    };
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
                            func: Var {
                                var: RawId(
                                    snd,
                                ),
                            },
                            args: [
                                App {
                                    func: Var {
                                        var: RawId(
                                            fst,
                                        ),
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
    "#]];
    let expect = expect;
    expect.assert_eq(&actual)
}
