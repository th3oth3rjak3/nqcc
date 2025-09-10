#[derive(Debug, Clone, PartialEq)]
pub enum T {
    Char,
    SChar,
    UChar,
    Int,
    Long,
    UInt,
    ULong,
    Double,
    Pointer(Box<T>),
    Void,
    Array { typ: Box<T>, size: i64 },
    FunType {},
    Structure(String),
}
