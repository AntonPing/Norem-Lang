use crate::common::lit::LitVal;
use crate::common::name::Name;

use super::typ::Type;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Lit {
        lit: LitVal,
    },
    /// x
    Var {
        var: Name,
    },
    /// let x = E1 in E2
    Let {
        bind: Name,
        expr: Box<Expr>,
        cont: Box<Expr>,
    },
    /// fn(x1: T1, ..., xn: Tn) => E
    Func {
        pars: Vec<(Name, Type)>,
        body: Box<Expr>,
    },
    /// E0(E1, ..., En)
    App {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    /// (E1, ..., En)
    Tup {
        flds: Vec<Expr>,
    },
    /// E.i
    Sel {
        expr: Box<Expr>,
        idx: usize,
    },
    /// decls D1 ... Dn in E end
    Decls {
        decls: Vec<Decl>,
        cont: Box<Expr>,
    },
    /// E[T1, ..., Tn]
    Inst {
        expr: Box<Expr>,
        typs: Vec<Type>,
    },
    /// [X1=T1, ..., Xn=Tn](E1, ..., En)
    Pack {
        seals: Vec<(Name, Type)>,
        flds: Vec<Expr>,
    },
    /// unpack x[X1, ..., Xn] = E1 in E2
    Unpack {
        bind: Name,
        opens: Vec<Name>,
        expr: Box<Expr>,
        cont: Box<Expr>,
    },
}

/// function f[X1, ..., Xn](x1: T1, ..., xn: Tn) -> U
/// begin
///     E
/// end
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Decl {
    pub name: Name,
    pub gens: Option<Vec<Name>>,
    pub pars: Vec<(Name, Type)>,
    pub res: Type,
    pub body: Expr,
}
