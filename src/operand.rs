#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operand {
    pub value: String,
}

impl Operand {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn from_bytecode(bytecode: &Vec<char>, start: usize, end: usize) -> Self {
        Self {
            value: bytecode[start..=end].iter().collect(),
        }
    }
}
