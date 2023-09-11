#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operand {
    pub value: String,
    pub size: usize, 
}

impl Operand {
    pub fn new(size: usize) -> Self {
        Self {
            value: String::new(),
            size: size, 
        }
    }

    pub fn set_value(&mut self, bytecode: &Vec<char>, start: usize, end: usize) {
        self.value = bytecode[start..end].iter().collect();
    }
}
