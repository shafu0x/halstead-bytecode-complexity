use crate::opcode::Opcode;
use crate::operand::Operand;
use std::collections::HashSet;

pub struct Stats {
    pub opcodes: Vec<Opcode>,
    pub operands: Vec<Operand>,
    pub opcode_count: usize,
    pub operand_count: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            operands: Vec::new(),
            opcode_count: 0,
            operand_count: 0,
        }
    }

    pub fn add_opcode(&mut self, opcode: Opcode) {
        self.operand_count += opcode.stack_input_size;
        self.opcode_count += 1;
        // TODO: refactor
        self.operands.push(opcode.operand.clone());
        self.opcodes.push(opcode);
    }

    pub fn count_unique_opcodes(&self) -> usize {
        self.opcodes
            .iter()
            .map(|opcode| &opcode.name)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn count_unique_operands(&self) -> usize {
        self.operands
            .iter()
            .map(|operand| &operand.value)
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn print(&self) {
        println!("");

        let unique_opcodes = self.count_unique_opcodes();
        let unique_operands = self.count_unique_operands();

        let vocabulary = unique_opcodes + unique_operands;
        println!("Vocabulary: {}", vocabulary);

        let length = self.opcodes.len() + self.operands.len();
        println!("Length:     {}", length);

        let volume = length as f64 * (vocabulary as f64).log2();
        println!("Volume:     {:.2}", volume);

        let difficulty =
            (unique_opcodes as f64) / 2.0 * (self.operands.len() as f64) / (unique_operands as f64);
        println!("Difficulty: {:.2}", difficulty);

        let effort = difficulty * volume;
        println!("Effort:     {:.2}", effort);
    }
}
