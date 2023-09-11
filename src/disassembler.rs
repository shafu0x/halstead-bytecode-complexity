use std::fs;

use crate::opcode::Opcode;
use crate::stats::Stats;

pub struct Disassembler {
    pub bytecode: Vec<char>,
    pub stats: Stats,
    pub line_number: usize,
    index: usize,
}

fn read_bytecode(path: &String, remove_metadata: bool) -> Result<Vec<char>, String> {
    let mut bytecode = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("Failed to read file: {}", e)),
    };

    // remove 0x if it exists
    if &bytecode[0..2] == "0x" {
        bytecode = bytecode[2..].to_string();
    }
    // remove metadata
    if remove_metadata {
        bytecode = strip_metadata(&bytecode);
    }

    Ok(bytecode.chars().collect())
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
            bytecode: read_bytecode(path, remove_metadata).expect("Error reading bytecode"),
            index: 0,
            line_number: 0,
            stats: Stats::new(),
        }
    }

    /// This function is responsible for parsing and extracting opcodes
    /// and operands from the bytecode.
    ///
    /// Reads the next opcode from the bytecode, advances the index,
    /// and returns the opcode as a Result.
    ///
    /// # Errors
    ///
    /// - If the function encounters the end of the bytecode, it returns
    ///   an `Err` with an "End of bytecode" message.
    ///
    /// # Returns
    ///
    /// - If the parsing is successful, it returns an `Ok` variant with
    ///   the parsed `Opcode`.
    pub fn next_opcode(&mut self) -> Result<Opcode, String> {
        if self.index >= self.bytecode.len() - 1 {
            return Err("End of bytecode".to_string());
        }

        let mut opcode_string = String::new();
        opcode_string.push(self.bytecode[self.index]);
        opcode_string.push(self.bytecode[self.index + 1]);
        self.index += 2;

        let mut opcode = Opcode::from_byte(&opcode_string);

        if opcode.operand.size > 0 {
            opcode.operand.set_value(
                &self.bytecode,
                self.index,
                self.index + (opcode.operand.size * 2),
            );
            self.index += opcode.operand.size * 2;
        }

        self.stats.add_opcode(opcode.clone());
        self.line_number += 1;

        Ok(opcode)
    }

    pub fn print_stats(&self) {
        self.stats.print();
    }
}
