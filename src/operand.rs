#[derive(Clone)]
pub struct Operand {
    pub value: String,
}

impl Operand {
    pub fn new(bytecode: &Vec<char>, start: usize, end: usize) -> Self {
        Self { 
            value: bytecode[start..=end]
                .iter()
                .collect(),
        }
    }
}
