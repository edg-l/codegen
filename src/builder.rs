use internment::ArcIntern;

use crate::ir::{
    self, Block, Function, FunctionType, IString, Inst, InstValue, Module, Type, Value,
};

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: ArcIntern::from_ref(name),
            functions: Default::default(),
        }
    }
}

impl Function {
    pub fn new(symbol: &str, ty: &FunctionType) -> Self {
        Function {
            symbol: ArcIntern::from_ref(symbol),
            ty: ty.clone(),
            blocks: vec![Block::new("entry")],
            cconv: crate::ir::CallConvention::CCC,
        }
    }
}

impl Block {
    pub fn new(label: &str) -> Self {
        Self {
            label: ArcIntern::from_ref(label),
            instructions: Vec::new(),
            label_counter: 0,
        }
    }

    fn get_label(&mut self, name: &str, hint: Option<&'static str>) -> IString {
        if name.is_empty() {
            IString::new(format!(
                "{}_{}{}",
                self.label,
                hint.unwrap_or_default(),
                self.label_counter
            ))
        } else {
            IString::from_ref(name)
        }
    }

    pub fn addi(&mut self, lhs: Value, rhs: Value, name: &str) -> Value {
        assert!(matches!(*lhs.ty, Type::Int(_)));
        assert_eq!(*lhs.ty, *rhs.ty); // todo as error
        let ty = lhs.ty.clone();

        let label = self.get_label(name, Some("addi"));
        self.instructions.push(InstValue {
            label: Some(label.clone()),
            inst: Inst::Binary(ir::Binary::Add(lhs, rhs)),
        });

        Value { label, ty }
    }
}

pub struct DebugBuilder {}

pub fn a() {}
