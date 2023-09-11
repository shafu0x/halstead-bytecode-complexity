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

    fn count_operands(&self) -> usize {
        self.opcodes
            .iter()
            .map(|opcode| opcode.stack_input_size)
            .sum()
    }

    fn count_unique_opcodes(&self) -> usize {
        self.opcodes
            .iter()
            .map(|opcode| &opcode.name)
            .collect::<HashSet<_>>()
            .len()
    }

    fn count_unique_operands(&self) -> usize {
        self.opcodes
            .iter()
            .map(|opcode| &opcode.operand.value)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn print(&self) {
        println!("");

        let unique_opcodes = self.count_unique_opcodes();
        let unique_operands = self.count_unique_operands();
        let total_opcodes = self.opcodes.len();
        let total_operands = self.count_operands();

        let vocabulary = unique_opcodes + unique_operands;
        println!("Vocabulary: {}", vocabulary);

        let length = total_opcodes + total_operands;
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
