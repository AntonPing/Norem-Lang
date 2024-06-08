use std::cell::RefCell;
use std::rc::Rc;

use super::core::{Expr, Type};
use crate::common::lit::LitVal;
use crate::common::name::Name;
use crate::common::prim::Prim;

use im::Vector;

type Env = Vector<(Name, Value)>;
fn lookup(env: &Env, key: &Name) -> Option<Value> {
    env.iter()
        .rev()
        .find(|(k, _v)| *k == *key)
        .map(|(_k, v)| v.clone())
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Lit(LitVal),
    Clos(Rc<RefCell<Env>>, Vec<(Name, Type)>, Rc<Expr>),
    Tup(Vec<Value>),
}

pub enum EvalError {
    Error,
}

pub fn eval(env: &mut Env, expr: &Expr) -> Result<Value, EvalError> {
    match expr {
        Expr::Lit { lit } => Ok(Value::Lit(*lit)),
        Expr::Var { var } => lookup(env, var).ok_or(EvalError::Error),
        Expr::Prim { prim, args } => {
            let args: Vec<Value> = args
                .iter()
                .map(|arg| eval(env, arg))
                .collect::<Result<Vec<_>, _>>()?;
            match (prim, &args[..]) {
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
                _ => Err(EvalError::Error),
            }
        }
        Expr::Let {
            bind,
            expr: body,
            cont,
        } => {
            let expr2 = eval(env, body)?;
            env.push_back((*bind, expr2));
            let cont2 = eval(env, cont)?;
            env.pop_back();
            Ok(cont2)
        }
        Expr::Func { pars, body } => Ok(Value::Clos(
            Rc::new(RefCell::new(env.clone())),
            pars.clone(),
            Rc::new(*body.clone()),
        )),
        Expr::App { func, args } => {
            let func2 = eval(env, func)?;
            if let Value::Clos(env2, pars, body) = func2 {
                if pars.len() == args.len() {
                    for ((par, _typ), arg) in pars.iter().zip(args.iter()) {
                        let arg2 = eval(env, arg)?;
                        env.push_back((*par, arg2));
                    }
                    let body2 = eval(&mut env2.borrow().clone(), &body)?;
                    for _ in pars {
                        env.pop_back();
                    }
                    Ok(body2)
                } else {
                    Err(EvalError::Error)
                }
            } else {
                Err(EvalError::Error)
            }
        }
        Expr::Tup { flds } => {
            let flds2 = flds
                .iter()
                .map(|fld| eval(env, fld))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Tup(flds2))
        }
        Expr::Sel { expr, idx } => {
            let expr2 = eval(env, expr)?;
            if let Value::Tup(flds) = expr2 {
                flds.iter().nth(*idx).cloned().ok_or(EvalError::Error)
            } else {
                Err(EvalError::Error)
            }
        }
        Expr::Letrec { decls, cont } => {
            let env2 = Rc::new(RefCell::new(env.clone()));
            for decl in decls {
                let clos = Value::Clos(env2.clone(), decl.pars.clone(), Rc::new(decl.body.clone()));
                env2.borrow_mut().push_back((decl.name, clos));
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
            let expr2 = eval(env, expr)?;
            env.push_back((*bind, expr2));
            let cont2 = eval(env, cont)?;
            env.pop_back();
            Ok(cont2)
        }
        Expr::Ifte { cond, trbr, flbr } => todo!(),
    }
}
