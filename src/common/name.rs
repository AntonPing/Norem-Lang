use super::intern::InternStr;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Name {
    RawId(InternStr),
    UniqId(InternStr, usize),
    GenId(char, usize),
}
