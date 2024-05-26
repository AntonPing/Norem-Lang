#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum LitType {
    TyInt,
    TyFloat,
    TyChar,
    TyBool,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LitVal {
    Int(i64),
    Float(f64),
    Char(char),
    Bool(bool),
}

impl LitVal {
    pub fn get_typ(&self) -> LitType {
        match self {
            LitVal::Int(_) => LitType::TyInt,
            LitVal::Float(_) => LitType::TyFloat,
            LitVal::Char(_) => LitType::TyChar,
            LitVal::Bool(_) => LitType::TyBool,
        }
    }
}