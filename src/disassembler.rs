use crate::opcode::Opcode;
use crate::operand::Operand;
use crate::stats::Stats;
use std::fs;

pub struct Disassembler {
    pub bytecode: Vec<char>,
    pub stats: Stats,
    pub line_number: usize,
    index: usize,
}

fn read_bytecode(path: &String, remove_metadata: bool) -> Vec<char> {
    let mut bytecode = fs::read_to_string(path).unwrap();

    // remove 0x if it exists
    if &bytecode[0..2] == "0x" {
        bytecode = bytecode[2..].to_string();
    }
    // remove metadata
    if remove_metadata {
        bytecode = strip_metadata(&bytecode);
    }

    bytecode.chars().collect()
}

// strip the metadata from the bytecode
fn strip_metadata(bytecode: &String) -> String {
    // metadata length is given by the last byte
    let last_byte: String = bytecode[bytecode.len() - 3..bytecode.len() - 1].to_string();
    let metadata_len = usize::from_str_radix(&last_byte, 16).unwrap();
    bytecode[0..bytecode.len() - (metadata_len * 2) - 4].to_string()
}

impl Disassembler {
    pub fn new(path: &String, remove_metadata: bool) -> Disassembler {
        Disassembler {
            bytecode: read_bytecode(path, remove_metadata),
            index: 0,
            line_number: 0,
            stats: Stats::new(),
        }
    }

    pub fn next_opcode(&mut self) -> Result<Opcode, String> {
        if self.index >= self.bytecode.len() - 1 {
            return Err("End of bytecode".to_string());
        }

        let mut opcode_string = String::new();
        opcode_string.push(self.bytecode[self.index]);
        opcode_string.push(self.bytecode[self.index + 1]);
        self.index += 2;

        let mut opcode = Opcode::from_byte(&opcode_string);

        if opcode.has_operand {
            let operand = Operand::from_bytecode(
                &self.bytecode,
                self.index,
                self.index + (opcode.operand_size * 2) - 1,
            );
            opcode.operand = operand;
            self.index += opcode.operand_size * 2;
        }

        self.stats.add_opcode(opcode.clone());
        self.line_number += 1;

        Ok(opcode)
    }

    pub fn print_stats(&self) {
        self.stats.print();
    }
}
