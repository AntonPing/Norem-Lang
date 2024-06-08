use std::cell::RefCell;
use std::rc::Rc;

use super::core::{Expr, Program, Type};
use crate::common::intern::InternStr;
use crate::common::lit::LitVal;
use crate::common::name::Name;
use crate::common::prim::{Compare, Prim};

use im::HashMap;

type Env = HashMap<Name, Value>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Lit(LitVal),
    Clos(Rc<RefCell<Env>>, Vec<(Name, Type)>, Rc<Expr>),
    Tup(Vec<Value>),
}

#[derive(Clone, Debug)]
pub enum EvalError {
    Error,
    ValVarNotInScope(Name),
    PrimGotStuck(Prim, Vec<Value>),
    AppNotAFunction(Value),
    AppLengthNotMatch(Vec<Value>, Vec<(Name, Type)>),
    SelNotATuple(Value),
    SelIndexOutOfBound(Value, usize),
    IfteNotABool(Value),
}

pub fn eval(env: &mut Env, expr: &Expr) -> Result<Value, EvalError> {
    match expr {
        Expr::Lit { lit } => Ok(Value::Lit(*lit)),
        Expr::Var { var } => env
            .get(var)
            .cloned()
            .ok_or(EvalError::ValVarNotInScope(*var)),
        Expr::Prim { prim, args } => {
            let args_val: Vec<Value> = args
                .iter()
                .map(|arg| eval(env, arg))
                .collect::<Result<Vec<_>, _>>()?;
            match (prim, &args_val[..]) {
                (Prim::INeg, &[Value::Lit(LitVal::Int(arg1))]) => {
                    Ok(Value::Lit(LitVal::Int(-arg1)))
                }
                (Prim::IAdd, &[Value::Lit(LitVal::Int(arg1)), Value::Lit(LitVal::Int(arg2))]) => {
                    Ok(Value::Lit(LitVal::Int(arg1 + arg2)))
                }
                (Prim::ISub, &[Value::Lit(LitVal::Int(arg1)), Value::Lit(LitVal::Int(arg2))]) => {
                    Ok(Value::Lit(LitVal::Int(arg1 - arg2)))
                }
                (Prim::IMul, &[Value::Lit(LitVal::Int(arg1)), Value::Lit(LitVal::Int(arg2))]) => {
                    Ok(Value::Lit(LitVal::Int(arg1 * arg2)))
                }
                (Prim::IDiv, &[Value::Lit(LitVal::Int(arg1)), Value::Lit(LitVal::Int(arg2))]) => {
                    Ok(Value::Lit(LitVal::Int(arg1 / arg2)))
                }
                (Prim::IRem, &[Value::Lit(LitVal::Int(arg1)), Value::Lit(LitVal::Int(arg2))]) => {
                    Ok(Value::Lit(LitVal::Int(arg1 % arg2)))
                }
                (
                    Prim::ICmp(cmp),
                    &[Value::Lit(LitVal::Int(arg1)), Value::Lit(LitVal::Int(arg2))],
                ) => match cmp {
                    Compare::Lt => Ok(Value::Lit(LitVal::Bool(arg1 < arg2))),
                    Compare::Le => Ok(Value::Lit(LitVal::Bool(arg1 <= arg2))),
                    Compare::Eq => Ok(Value::Lit(LitVal::Bool(arg1 == arg2))),
                    Compare::Ne => Ok(Value::Lit(LitVal::Bool(arg1 != arg2))),
                    Compare::Ge => Ok(Value::Lit(LitVal::Bool(arg1 >= arg2))),
                    Compare::Gt => Ok(Value::Lit(LitVal::Bool(arg1 > arg2))),
                },
                (Prim::FNeg, &[Value::Lit(LitVal::Float(arg1))]) => {
                    Ok(Value::Lit(LitVal::Float(-arg1)))
                }
                (
                    Prim::FAdd,
                    &[Value::Lit(LitVal::Float(arg1)), Value::Lit(LitVal::Float(arg2))],
                ) => Ok(Value::Lit(LitVal::Float(arg1 + arg2))),
                (
                    Prim::FSub,
                    &[Value::Lit(LitVal::Float(arg1)), Value::Lit(LitVal::Float(arg2))],
                ) => Ok(Value::Lit(LitVal::Float(arg1 - arg2))),
                (
                    Prim::FMul,
                    &[Value::Lit(LitVal::Float(arg1)), Value::Lit(LitVal::Float(arg2))],
                ) => Ok(Value::Lit(LitVal::Float(arg1 * arg2))),
                (
                    Prim::FDiv,
                    &[Value::Lit(LitVal::Float(arg1)), Value::Lit(LitVal::Float(arg2))],
                ) => Ok(Value::Lit(LitVal::Float(arg1 / arg2))),
                (
                    Prim::FCmp(cmp),
                    &[Value::Lit(LitVal::Float(arg1)), Value::Lit(LitVal::Float(arg2))],
                ) => match cmp {
                    Compare::Lt => Ok(Value::Lit(LitVal::Bool(arg1 < arg2))),
                    Compare::Le => Ok(Value::Lit(LitVal::Bool(arg1 <= arg2))),
                    Compare::Eq => Ok(Value::Lit(LitVal::Bool(arg1 == arg2))),
                    Compare::Ne => Ok(Value::Lit(LitVal::Bool(arg1 != arg2))),
                    Compare::Ge => Ok(Value::Lit(LitVal::Bool(arg1 >= arg2))),
                    Compare::Gt => Ok(Value::Lit(LitVal::Bool(arg1 > arg2))),
                },
                (
                    Prim::CCmp(cmp),
                    &[Value::Lit(LitVal::Char(arg1)), Value::Lit(LitVal::Char(arg2))],
                ) => match cmp {
                    Compare::Lt => Ok(Value::Lit(LitVal::Bool(arg1 < arg2))),
                    Compare::Le => Ok(Value::Lit(LitVal::Bool(arg1 <= arg2))),
                    Compare::Eq => Ok(Value::Lit(LitVal::Bool(arg1 == arg2))),
                    Compare::Ne => Ok(Value::Lit(LitVal::Bool(arg1 != arg2))),
                    Compare::Ge => Ok(Value::Lit(LitVal::Bool(arg1 >= arg2))),
                    Compare::Gt => Ok(Value::Lit(LitVal::Bool(arg1 > arg2))),
                },
                (Prim::IScan | Prim::FScan | Prim::CScan, &[]) => {
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let res = match prim {
                        Prim::IScan => {
                            LitVal::Int(input.trim().parse().expect("Input is not an integer!"))
                        }
                        Prim::FScan => LitVal::Float(
                            input
                                .trim()
                                .parse()
                                .expect("Input is not a floating number!"),
                        ),
                        Prim::CScan => {
                            LitVal::Char(input.trim().parse().expect("Input is not a charactor!"))
                        }
                        _ => unreachable!(),
                    };
                    Ok(Value::Lit(res))
                }
                (Prim::IPrint, &[Value::Lit(LitVal::Int(arg1))]) => {
                    println!("{}", arg1);
                    Ok(Value::Lit(LitVal::Unit))
                }
                (Prim::FPrint, &[Value::Lit(LitVal::Float(arg1))]) => {
                    println!("{}", arg1);
                    Ok(Value::Lit(LitVal::Unit))
                }
                (Prim::CPrint, &[Value::Lit(LitVal::Char(arg1))]) => {
                    println!("{}", arg1);
                    Ok(Value::Lit(LitVal::Unit))
                }
                (prim, _) => Err(EvalError::PrimGotStuck(*prim, args_val.clone())),
            }
        }
        Expr::Let { bind, expr, cont } => {
            let expr_val = eval(env, expr)?;
            let mut env2 = env.clone();
            env2.insert(*bind, expr_val);
            let cont_val = eval(&mut env2, cont)?;
            Ok(cont_val)
        }
        Expr::Func { pars, body } => Ok(Value::Clos(
            Rc::new(RefCell::new(env.clone())),
            pars.clone(),
            Rc::new(*body.clone()),
        )),
        Expr::App { func, args } => {
            let func = eval(env, func)?;
            let args_val: Vec<Value> = args
                .iter()
                .map(|arg| eval(env, arg))
                .collect::<Result<_, _>>()?;
            if let Value::Clos(env2, pars, body) = func {
                if pars.len() == args_val.len() {
                    let mut env3 = env2.borrow().clone();
                    for ((par, _typ), arg) in pars.iter().zip(args.iter()) {
                        let arg_val = eval(env, arg)?;
                        env3.insert(*par, arg_val);
                    }
                    let body_val = eval(&mut env3, &body)?;
                    Ok(body_val)
                } else {
                    Err(EvalError::AppLengthNotMatch(args_val, pars))
                }
            } else {
                Err(EvalError::AppNotAFunction(func))
            }
        }
        Expr::Tup { flds } => {
            let flds_val = flds
                .iter()
                .map(|fld| eval(env, fld))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Tup(flds_val))
        }
        Expr::Sel { expr, idx } => {
            let expr_val = eval(env, expr)?;
            if let Value::Tup(flds) = expr_val {
                flds.iter().nth(*idx).cloned().ok_or(EvalError::Error)
            } else {
                Err(EvalError::SelNotATuple(expr_val))
            }
        }
        Expr::Letrec { decls, cont } => {
            let env2 = Rc::new(RefCell::new(env.clone()));
            for decl in decls {
                let clos = Value::Clos(env2.clone(), decl.pars.clone(), Rc::new(decl.body.clone()));
                env2.borrow_mut().insert(decl.name, clos);
            }
            let mut env3 = env2.borrow().clone();
            eval(&mut env3, cont)
        }
        Expr::Inst { expr, typs: _ } => eval(env, expr),
        Expr::Pack {
            expr,
            seals: _,
            flds: _,
        } => eval(env, expr),
        Expr::Unpack {
            bind,
            opens: _,
            expr,
            cont,
        } => {
            let expr_val = eval(env, expr)?;
            let mut env2 = env.clone();
            env2.insert(*bind, expr_val);
            let cont_val = eval(&mut env2, cont)?;
            Ok(cont_val)
        }
        Expr::Ifte { cond, trbr, flbr } => {
            let cond_val = eval(env, cond)?;
            match cond_val {
                Value::Lit(LitVal::Bool(true)) => eval(env, trbr),
                Value::Lit(LitVal::Bool(false)) => eval(env, flbr),
                _ => Err(EvalError::IfteNotABool(cond_val)),
            }
        }
    }
}

pub fn eval_prog(prog: &Program) -> Result<Value, EvalError> {
    let env = Rc::new(RefCell::new(HashMap::new()));
    let mut has_entry = false;
    for decl in prog.decls.iter() {
        let clos = Value::Clos(env.clone(), decl.pars.clone(), Rc::new(decl.body.clone()));
        env.borrow_mut().insert(decl.name, clos);
        if decl.name == Name::RawId(InternStr::new("main")) {
            has_entry = true;
        }
    }
    if has_entry {
        eval(
            &mut env.borrow().clone(),
            &Expr::App {
                func: Box::new(Expr::Var {
                    var: Name::RawId(InternStr::new("main")),
                }),
                args: Vec::new(),
            },
        )
    } else {
        Ok(Value::Lit(LitVal::Unit))
    }
}
