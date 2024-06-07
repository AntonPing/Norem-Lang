use super::core::{Decl, Expr, Program, Type};
use crate::common::name::Name;
use im::{HashMap, HashSet};

type TyEnv = HashSet<Name>;
type Env = HashMap<Name, Type>;

#[derive(Clone, Debug)]
pub enum InferError {
    ValVarNotInScope(Name),
    TypeVarNotInScope(Name),
    AppNotAFunction(Expr),
    AppLengthNotMatch(Vec<Expr>, Vec<Type>),
    SelNotATuple(Expr),
    SelIndexOutOfBound(Expr, usize),
    InstNotAForall(Expr),
    InstLengthNotMatch(Vec<Type>, Vec<Name>),
    PackNotATuple(Expr),
    PackLengthNotMatch(Vec<Type>, Vec<Type>),
    UnpackNotAExist(Expr),
    UnpackLengthNotMatch(Vec<Name>, Vec<Name>),
    CantUnifyType(Type, Type),
}

pub fn check_typ(tyenv: &mut TyEnv, typ: &Type) -> Result<(), InferError> {
    match typ {
        Type::Lit { .. } => Ok(()),
        Type::Var { var } => {
            if tyenv.contains(var) {
                Ok(())
            } else {
                Err(InferError::TypeVarNotInScope(*var))
            }
        }
        Type::Func { pars, res } => {
            for typ in pars.iter() {
                check_typ(tyenv, typ)?;
            }
            check_typ(tyenv, res)
        }
        Type::Tup { flds } => {
            for fld in flds.iter() {
                check_typ(tyenv, fld)?;
            }
            Ok(())
        }
        Type::Forall { gens, pars, res } => {
            let mut tyenv2 = tyenv.clone();
            for gen in gens.iter() {
                tyenv2.insert(*gen);
            }
            for typ in pars.iter() {
                check_typ(&mut tyenv2, typ)?;
            }
            check_typ(&mut tyenv2, res)
        }
        Type::Exist { seals, flds } => {
            let mut tyenv2 = tyenv.clone();
            for seal in seals.iter() {
                tyenv2.insert(*seal);
            }
            for fld in flds.iter() {
                check_typ(tyenv, fld)?;
            }
            Ok(())
        }
    }
}

pub fn check_expr(tyenv: &mut TyEnv, env: &mut Env, expr: &Expr) -> Result<Type, InferError> {
    match expr {
        Expr::Lit { lit } => Ok(Type::Lit { lit: lit.get_typ() }),
        Expr::Var { var } => env
            .get(&var)
            .cloned()
            .ok_or(InferError::ValVarNotInScope(*var)),
        Expr::Let { bind, expr, cont } => {
            let expr_ty = check_expr(tyenv, env, expr)?;
            let mut env2 = env.clone();
            env2.insert(*bind, expr_ty);
            let cont_ty = check_expr(tyenv, &mut env2, cont)?;
            Ok(cont_ty)
        }
        Expr::Func { pars, body } => {
            let mut env2 = env.clone();
            for (par, typ) in pars {
                check_typ(tyenv, typ)?;
                env2.insert(*par, typ.clone());
            }
            let body_ty = check_expr(tyenv, &mut env2, body)?;
            Ok(body_ty)
        }
        Expr::App { func, args } => {
            let func_ty = check_expr(tyenv, env, func)?;
            if let Type::Func { pars, res } = func_ty {
                if pars.len() == args.len() {
                    for (par, arg) in pars.iter().zip(args.iter()) {
                        let arg_ty = check_expr(tyenv, env, arg)?;
                        if *par != arg_ty {
                            return Err(InferError::CantUnifyType(par.clone(), arg_ty.clone()));
                        }
                    }
                    Ok(*res)
                } else {
                    Err(InferError::AppLengthNotMatch(args.clone(), pars.clone()))
                }
            } else {
                Err(InferError::AppNotAFunction(*func.clone()))
            }
        }
        Expr::Tup { flds } => {
            let flds = flds
                .iter()
                .map(|fld| check_expr(tyenv, env, fld))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Type::Tup { flds })
        }
        Expr::Sel { expr, idx } => {
            let expr_ty = check_expr(tyenv, env, expr)?;
            if let Type::Tup { flds } = expr_ty {
                if *idx < flds.len() {
                    Ok(flds.into_iter().nth(*idx).unwrap())
                } else {
                    Err(InferError::SelIndexOutOfBound(*expr.clone(), *idx))
                }
            } else {
                Err(InferError::SelNotATuple(*expr.clone()))
            }
        }
        Expr::Letrec { decls, cont } => {
            let mut env2 = env.clone();
            for decl in decls.iter() {
                let typ = get_decl_type(decl);
                check_typ(tyenv, &typ)?;
                env2.insert(decl.name, typ);
            }
            for decl in decls.iter() {
                check_decl(tyenv, &mut env2, decl)?;
            }
            let cont_ty = check_expr(tyenv, &mut env2, cont)?;
            Ok(cont_ty)
        }
        Expr::Inst { expr, typs } => {
            let expr_ty = check_expr(tyenv, env, expr)?;
            for typ in typs.iter() {
                check_typ(tyenv, typ)?;
            }
            if let Type::Forall { gens, pars, res } = expr_ty {
                if gens.len() == typs.len() {
                    let mut map: HashMap<Name, Type> =
                        gens.iter().cloned().zip(typs.iter().cloned()).collect();
                    let pars = pars.iter().map(|par| subst(&mut map, par)).collect();
                    let res = Box::new(subst(&mut map, &res));
                    Ok(Type::Func { pars, res })
                } else {
                    Err(InferError::InstLengthNotMatch(typs.clone(), gens.clone()))
                }
            } else {
                Err(InferError::InstNotAForall(*expr.clone()))
            }
        }
        Expr::Pack { expr, seals, flds } => {
            let expr_ty = check_expr(tyenv, env, expr)?;
            let mut tyenv2 = tyenv.clone();
            for (seal, typ) in seals.iter() {
                check_typ(tyenv, typ)?;
                tyenv2.insert(*seal);
            }
            for fld in flds.iter() {
                check_typ(&mut tyenv2, fld)?;
            }
            match expr_ty {
                Type::Tup { flds: flds2 } => {
                    if flds.len() == flds2.len() {
                        let mut map: HashMap<Name, Type> = seals.iter().cloned().collect();
                        for (fld, fld2) in flds.iter().zip(flds2.iter()) {
                            let subst_fld = subst(&mut map, fld);
                            if subst_fld != *fld2 {
                                return Err(InferError::CantUnifyType(subst_fld, fld2.clone()));
                            }
                        }
                        Ok(Type::Exist {
                            seals: seals.iter().map(|(x, _)| *x).collect(),
                            flds: flds.clone(),
                        })
                    } else {
                        Err(InferError::PackLengthNotMatch(flds2.clone(), flds.clone()))
                    }
                }
                _ => Err(InferError::PackNotATuple(*expr.clone())),
            }
        }
        Expr::Unpack {
            bind,
            opens,
            expr,
            cont,
        } => {
            let expr_ty = check_expr(tyenv, env, expr)?;

            if let Type::Exist { seals, flds } = expr_ty {
                if opens.len() == seals.len() {
                    let mut map = opens
                        .iter()
                        .cloned()
                        .zip(seals.iter().map(|x| Type::Var { var: *x }))
                        .collect();
                    let flds = flds.iter().map(|fld| subst(&mut map, fld)).collect();

                    let mut tyenv2 = tyenv.clone();
                    let mut env2 = env.clone();
                    for open in opens {
                        tyenv2.insert(*open);
                    }
                    env2.insert(*bind, Type::Tup { flds });
                    let cont = check_expr(&mut tyenv2, &mut env2, cont)?;
                    Ok(cont)
                } else {
                    Err(InferError::UnpackLengthNotMatch(
                        opens.clone(),
                        seals.clone(),
                    ))
                }
            } else {
                Err(InferError::UnpackNotAExist(*expr.clone()))
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

pub fn check_decl(tyenv: &mut TyEnv, env: &mut Env, decl: &Decl) -> Result<(), InferError> {
    let mut tyenv2 = tyenv.clone();
    let mut env2 = env.clone();
    if let Some(gens) = &decl.gens {
        for gen in gens.iter() {
            tyenv2.insert(*gen);
        }
    }
    for (par, typ) in decl.pars.iter() {
        check_typ(&mut tyenv2, typ)?;
        env2.insert(*par, typ.clone());
    }
    check_typ(&mut tyenv2, &decl.res)?;
    let body_ty = check_expr(&mut tyenv2, &mut env2, &decl.body)?;
    if body_ty == decl.res {
        Ok(())
    } else {
        Err(InferError::CantUnifyType(body_ty, decl.res.clone()))
    }
}

pub fn check_prog(prog: &Program) -> Result<(), InferError> {
    let mut tyenv = HashSet::new();
    let mut env = HashMap::new();
    for decl in prog.decls.iter() {
        let typ = get_decl_type(decl);
        check_typ(&mut tyenv, &typ)?;
        env.insert(decl.name, typ);
    }
    for decl in prog.decls.iter() {
        check_decl(&mut tyenv, &mut env, decl)?;
    }
    Ok(())
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
