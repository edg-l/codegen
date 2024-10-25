use std::collections::BTreeMap;

use internment::ArcIntern;

/// Interned String
pub(crate)  type IString = ArcIntern<String>;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Module {
    pub name: IString,
    pub functions: BTreeMap<IString, Function>,
    // globals, etc
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Function {
    pub symbol: IString,
    pub ty: FunctionType,
    pub blocks: Vec<Block>,
    pub cconv: CallConvention,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallConvention {
    CCC,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Linkage {
    Private,
    Internal,
    External,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Block {
    pub label: IString,
    pub instructions: Vec<InstValue>,
    pub(crate)  label_counter: usize,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct InstValue {
    pub inst: Inst,
    pub label: Option<IString>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Inst {
    Terminator(Terminator),
    Binary(Binary),
    BitwiseBinary(BitwiseBinary),
    Memory(Memory),
    Conversion(Conversion),
    Const()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConstValue {
    Int(),
    Br,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Terminator {
    Ret,
    Br,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Binary {
    Add(Value, Value),
    FAdd,
    Sub,
    FSub,
    Mul,
    UDiv,
    SDiv,
    URem,
    SRem,
    FRem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitwiseBinary {
    Shl,
    LShr,
    AShr,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Memory {
    Alloca,
    Load,
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Conversion {
    Trunc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntType {
    pub size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FloatType {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PtrType {
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructType {
    pub fields: Vec<Type>,
    pub packed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayType {
    pub of: Box<Type>,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret_ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Int(IntType),
    Float(FloatType),
    Struct(StructType),
    Array(ArrayType),
    Ptr(PtrType),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct Value {
    pub label: IString,
    pub ty: ArcIntern<Type>,
}
