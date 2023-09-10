use crate::opcode::Opcode;

pub struct Stats {
    pub opcodes: Vec<Opcode>,
    pub opcode_count: usize,
    pub operand_count: usize,
}

impl Stats {
    pub fn new(opcode_count: usize, operand_count: usize) -> Self {
        Self {
            opcodes: Vec::new(),
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
}
