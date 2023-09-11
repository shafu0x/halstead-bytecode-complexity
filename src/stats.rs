use crate::opcode::Opcode;
use std::collections::HashSet;

pub struct Stats {
    pub opcodes: Vec<Opcode>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
        }
    }

    pub fn add_opcode(&mut self, opcode: Opcode) {
        self.opcodes.push(opcode);
    }

    pub fn get_opcodes_with_operand(&self) -> Vec<Opcode> {
        self.opcodes
            .iter()
            .filter(|opcode| opcode.has_operand)
            .cloned()
            .collect()
    }

    pub fn count_unique_opcodes(&self) -> usize {
        self.opcodes
            .iter()
            .map(|opcode| &opcode.name)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn count_unique_operands(&self) -> usize {
        self.get_opcodes_with_operand()
            .iter()
            .map(|opcode| &opcode.operand.value)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn print(&self) {
        println!("");

        let unique_opcodes = self.count_unique_opcodes();
        let unique_operands = self.count_unique_operands();
        let total_operands = self.get_opcodes_with_operand().len();

        let vocabulary = unique_opcodes + unique_operands;
        println!("Vocabulary: {}", vocabulary);

        let length = self.opcodes.len() + total_operands;
        println!("Length:     {}", length);

        let volume = length as f64 * (vocabulary as f64).log2();
        println!("Volume:     {:.2}", volume);

        let difficulty =
            (unique_opcodes as f64) / 2.0 * (total_operands as f64) / (unique_operands as f64);
        println!("Difficulty: {:.2}", difficulty);

        let effort = difficulty * volume;
        println!("Effort:     {:.2}", effort);
    }
}
