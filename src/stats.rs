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
    pub fn new(opcode_count: usize, operand_count: usize) -> Self {
        Self {
            opcodes: Vec::new(),
            operands: Vec::new(),
            opcode_count,
            operand_count,
        }
    }

    pub fn inc_opcode_count(&mut self) {
        self.opcode_count += 1;
    }

    pub fn inc_operand_count(&mut self, count: usize) {
        self.operand_count += count;
    }

    pub fn add_opcode(&mut self, opcode: Opcode) {
        self.opcodes.push(opcode);
    }

    pub fn add_operand(&mut self, operand: Operand) {
        self.operands.push(operand);
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
}
