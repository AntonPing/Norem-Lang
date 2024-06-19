use crate::common::lit::{LitType, LitVal};
use crate::common::name::Name;
use crate::common::prim::Prim;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Lit {
        lit: LitVal,
    },
    /// x
    Var {
        var: Name,
    },
    /// @prim(E1, ..., En)
    Prim {
        prim: Prim,
        args: Vec<Expr>,
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
    /// letrec D1 ... Dn in E end
    Letrec {
        decls: Vec<Decl>,
        cont: Box<Expr>,
    },
    /// E[T1, ..., Tn]
    Inst {
        expr: Box<Expr>,
        typs: Vec<Type>,
    },
    /// pack E as [X1=T1, ..., Xn=Tn](U1, ..., Un)
    Pack {
        expr: Box<Expr>,
        seals: Vec<(Name, Type)>,
        flds: Vec<Type>,
    },
    /// unpack x[X1, ..., Xn] = E1 in E2
    Unpack {
        bind: Name,
        opens: Vec<Name>,
        expr: Box<Expr>,
        cont: Box<Expr>,
    },
    /// if E1 then E2 else E3
    Ifte {
        cond: Box<Expr>,
        trbr: Box<Expr>,
        flbr: Box<Expr>,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Lit {
        lit: LitType,
    },
    /// x
    Var {
        var: Name,
    },
    /// fn(T1, ..., Tn) -> U
    Func {
        pars: Vec<Type>,
        res: Box<Type>,
    },
    /// (T1, ..., Tn)
    Tup {
        flds: Vec<Type>,
    },
    /// TC[T1, ..., Tn]
    Cons {
        cons: Name,
        args: Vec<Type>,
    },
    /// fn[X1, ..., Xn](T1, ..., Tn) -> U
    Forall {
        gens: Vec<Name>,
        pars: Vec<Type>,
        res: Box<Type>,
    },
    /// [X1, ..., Xn](T1, ..., Tn)
    Exist {
        seals: Vec<Name>,
        flds: Vec<Type>,
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

/// datatype D[T1, ..., Tn] where
/// | C1 { x1: T1, ..., xn: Tn }
/// | ......
/// | Cn { x1: T1, ..., xn: Tn }
/// end
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Data {
    pub name: Name,
    pub gens: Vec<Name>,
    pub cons: Vec<Cons>,
}

/// C { x1: T1, ..., xn: Tn }
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Cons {
    pub name: Name,
    pub flds: Vec<(Name, Type)>,
}

/// toplevel program
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Program {
    pub datas: Vec<Data>,
    pub decls: Vec<Decl>,
}
