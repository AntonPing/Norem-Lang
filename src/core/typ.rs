use crate::common::lit::LitType;
use crate::common::name::Name;

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
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
