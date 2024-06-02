use crate::common::name::Name;

use super::core::{Decl, Expr, Type};

use im::HashMap;
use im::Vector;

type Env = Vector<(Name, Type)>;

fn lookup(env: &Env, key: &Name) -> Option<Type> {
    env.iter()
        .rev()
        .find(|(k, _v)| *k == *key)
        .map(|(_k, v)| v.clone())
}

pub enum InferError {
    Error,
}

pub fn check(env: &mut Env, expr: &Expr) -> Result<Type, InferError> {
    match expr {
        Expr::Lit { lit } => Ok(Type::Lit { lit: lit.get_typ() }),
        Expr::Var { var } => lookup(&env, &var).ok_or(InferError::Error),
        Expr::Let { bind, expr, cont } => {
            let expr_ty = check(env, expr)?;
            env.push_back((*bind, expr_ty));
            let cont_ty = check(env, cont)?;
            env.pop_back();
            Ok(cont_ty)
        }
        Expr::Func { pars, body } => {
            for (par, typ) in pars {
                env.push_back((*par, typ.clone()));
            }
            let body_ty = check(env, body)?;
            for _ in pars {
                env.pop_back();
            }
            Ok(body_ty)
        }
        Expr::App { func, args } => {
            let func_ty = check(env, func)?;
            if let Type::Func { pars, res } = func_ty {
                if pars.len() == args.len() {
                    for (par, arg) in pars.iter().zip(args.iter()) {
                        let arg = check(env, arg)?;
                        if *par != arg {
                            return Err(InferError::Error);
                        }
                    }
                    Ok(*res)
                } else {
                    Err(InferError::Error)
                }
            } else {
                Err(InferError::Error)
            }
        }
        Expr::Tup { flds } => {
            let flds = flds
                .iter()
                .map(|fld| check(env, fld))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Type::Tup { flds })
        }
        Expr::Sel { expr, idx } => {
            let expr = check(env, expr)?;
            if let Type::Tup { flds } = expr {
                if *idx < flds.len() {
                    Ok(flds.into_iter().nth(*idx).unwrap())
                } else {
                    Err(InferError::Error)
                }
            } else {
                Err(InferError::Error)
            }
        }
        Expr::Letrec { decls, cont } => {
            for decl in decls.iter() {
                env.push_back((decl.name, get_decl_type(decl)));
            }
            for decl in decls.iter() {
                check_decl(env, decl)?;
            }
            let cont = check(env, cont)?;
            for _ in decls.iter() {
                env.pop_back();
            }
            Ok(cont)
        }
        Expr::Inst { expr, typs } => {
            let expr = check(env, expr)?;
            if let Type::Forall { gens, pars, res } = expr {
                if gens.len() == typs.len() {
                    let mut map: HashMap<Name, Type> =
                        gens.iter().cloned().zip(typs.iter().cloned()).collect();
                    let pars = pars.iter().map(|par| subst(&mut map, par)).collect();
                    let res = Box::new(subst(&mut map, &res));
                    Ok(Type::Func { pars, res })
                } else {
                    Err(InferError::Error)
                }
            } else {
                Err(InferError::Error)
            }
        }
        Expr::Pack { expr, seals, flds } => {
            let expr = check(env, expr)?;
            match expr {
                Type::Tup { flds: flds2 } => {
                    if flds.len() == flds2.len() {
                        let mut map: HashMap<Name, Type> = seals.iter().cloned().collect();
                        for (fld, fld2) in flds.iter().zip(flds2.iter()) {
                            if subst(&mut map, fld) != *fld2 {
                                return Err(InferError::Error);
                            }
                        }
                        Ok(Type::Exist {
                            seals: seals.iter().map(|(x, _)| *x).collect(),
                            flds: flds.clone(),
                        })
                    } else {
                        Err(InferError::Error)
                    }
                }
                _ => Err(InferError::Error),
            }
        }
        Expr::Unpack {
            bind,
            opens,
            expr,
            cont,
        } => {
            let expr = check(env, expr)?;
            if let Type::Exist { seals, flds } = expr {
                if opens.len() == seals.len() {
                    let mut map = opens
                        .iter()
                        .cloned()
                        .zip(seals.iter().map(|x| Type::Var { var: *x }))
                        .collect();
                    let flds = flds.iter().map(|fld| subst(&mut map, fld)).collect();
                    env.push_back((*bind, Type::Tup { flds }));
                    let cont = check(env, cont)?;
                    env.pop_back();
                    Ok(cont)
                } else {
                    Err(InferError::Error)
                }
            } else {
                Err(InferError::Error)
            }
        }
    }
}

pub fn get_decl_type(decl: &Decl) -> Type {
    let pars = decl.pars.iter().map(|par| par.1.clone()).collect();
    let res = Box::new(decl.res.clone());
    match &decl.gens {
        Some(gens) => Type::Forall {
            gens: gens.clone(),
            pars,
            res,
        },
        None => Type::Func { pars, res },
    }
}

pub fn check_decl(env: &mut Env, decl: &Decl) -> Result<(), InferError> {
    for (par, typ) in decl.pars.iter() {
        env.push_back((*par, typ.clone()));
    }
    let body = check(env, &decl.body)?;
    for _ in decl.pars.iter() {
        env.pop_back();
    }
    if body == decl.res {
        Ok(())
    } else {
        Err(InferError::Error)
    }
}

pub fn subst(map: &mut HashMap<Name, Type>, typ: &Type) -> Type {
    match typ {
        Type::Lit { lit: _ } => typ.clone(),
        Type::Var { var } => {
            if let Some(res) = map.get(&var) {
                res.clone()
            } else {
                typ.clone()
            }
        }
        Type::Func { pars, res } => {
            let pars = pars.iter().map(|par| subst(map, par)).collect();
            let res = Box::new(subst(map, res));
            Type::Func { pars, res }
        }
        Type::Tup { flds } => {
            let flds = flds.iter().map(|fld| subst(map, fld)).collect();
            Type::Tup { flds }
        }
        Type::Forall { gens, pars, res } => {
            let mut map2 = map.clone();
            for gen in gens.iter() {
                map2.remove(gen);
            }
            let pars = pars.iter().map(|par| subst(&mut map2, par)).collect();
            let res = Box::new(subst(&mut map2, res));
            Type::Forall {
                gens: gens.clone(),
                pars,
                res,
            }
        }
        Type::Exist { seals, flds } => {
            let mut map2 = map.clone();
            for seal in seals.iter() {
                map2.remove(seal);
            }
            let flds = flds.iter().map(|fld| subst(&mut map2, fld)).collect();
            Type::Exist {
                seals: seals.clone(),
                flds,
            }
        }
    }
}
