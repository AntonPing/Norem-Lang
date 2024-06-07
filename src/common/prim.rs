use super::lit::LitType;
use crate::core::core::Type;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Prim {
    /// fn(Int) -> Int
    INeg,
    /// fn(Int, Int) -> Int
    IAdd,
    ISub,
    IMul,
    IDiv,
    IRem,
    /// fn(Float) -> Float
    FNeg,
    /// fn(Float, Float) -> Float
    FAdd,
    FSub,
    FMul,
    FDiv,
    /// fn(Bool) -> Bool
    BNot,
    /// fn(Bool, Bool) -> Bool
    BAnd,
    BOr,
    BXor,
    /// fn() -> Int (with IO effect)
    IScan,
    /// fn() -> Float (with IO effect)
    FScan,
    /// fn() -> Int (with IO effect)
    CScan,
    /// fn(Int) -> () (with IO efffect)
    IPrint,
    /// fn(Float) -> () (with IO efffect)
    FPrint,
    /// fn(Char) -> () (with IO efffect)
    CPrint,
}

fn unop(lit: LitType) -> Type {
    Type::Func {
        pars: vec![Type::Lit { lit }],
        res: Box::new(Type::Lit { lit }),
    }
}

fn binop(lit: LitType) -> Type {
    Type::Func {
        pars: vec![Type::Lit { lit }, Type::Lit { lit }],
        res: Box::new(Type::Lit { lit }),
    }
}

fn scan(lit: LitType) -> Type {
    Type::Func {
        pars: vec![],
        res: Box::new(Type::Lit { lit }),
    }
}

fn print(lit: LitType) -> Type {
    Type::Func {
        pars: vec![Type::Lit { lit }],
        res: Box::new(Type::Lit {
            lit: LitType::TyUnit,
        }),
    }
}

impl Prim {
    pub fn get_type(&self) -> Type {
        match self {
            Prim::INeg => unop(LitType::TyInt),
            Prim::IAdd => binop(LitType::TyInt),
            Prim::ISub => binop(LitType::TyInt),
            Prim::IMul => binop(LitType::TyInt),
            Prim::IDiv => binop(LitType::TyInt),
            Prim::IRem => binop(LitType::TyInt),
            Prim::FNeg => unop(LitType::TyFloat),
            Prim::FAdd => binop(LitType::TyFloat),
            Prim::FSub => binop(LitType::TyFloat),
            Prim::FMul => binop(LitType::TyFloat),
            Prim::FDiv => binop(LitType::TyFloat),
            Prim::BNot => unop(LitType::TyBool),
            Prim::BAnd => binop(LitType::TyBool),
            Prim::BOr => binop(LitType::TyBool),
            Prim::BXor => binop(LitType::TyBool),
            Prim::IScan => scan(LitType::TyInt),
            Prim::FScan => scan(LitType::TyFloat),
            Prim::CScan => scan(LitType::TyChar),
            Prim::IPrint => print(LitType::TyInt),
            Prim::FPrint => print(LitType::TyFloat),
            Prim::CPrint => print(LitType::TyChar),
        }
    }
}
