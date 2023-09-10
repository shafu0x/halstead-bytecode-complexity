mod opcode;
mod stats;

use std::env;
use std::fs;
use std::io;

use opcode::Opcode;
use stats::Stats;

const METADATA_FLAG: &str = "--rm-metadata";

// strip the metadata from the bytecode
fn strip_metadata(bytecode: &String) -> String {
    // metadata length is given by the last byte
    let last_byte: String = bytecode[bytecode.len() - 3..bytecode.len() - 1].to_string();
    let metadata_len = usize::from_str_radix(&last_byte, 16).unwrap();
    bytecode[0..bytecode.len() - (metadata_len * 2) - 4].to_string()
}

// read the bytecode from the file
fn read_bytecode() -> Result<Vec<char>, io::Error> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut bytecode = fs::read_to_string(path)?;
    // remove 0x if it exists
    if &bytecode[0..2] == "0x" {
        bytecode = bytecode[2..].to_string();
    }
    // remove metadata
    if args.len() > 2 && &args[2] == METADATA_FLAG {
        bytecode = strip_metadata(&bytecode);
    }

    Ok(bytecode.chars().collect())
}

fn print_metrics(stats: &Stats) {
    println!("");

    let unique_opcodes = stats.count_unique_opcodes();
    let unique_operands = stats.count_unique_operands();

    let vocabulary = unique_opcodes + unique_operands;
    println!("Vocabulary: {}", vocabulary);

    let length = stats.opcodes.len() + stats.operands.len();
    println!("Length:     {}", length);

    let volume = length as f64 * (vocabulary as f64).log2();
    println!("Volume:     {:.2}", volume);

    let difficulty =
        (unique_opcodes as f64) / 2.0 * (stats.operands.len() as f64) / (unique_operands as f64);
    println!("Difficulty: {:.2}", difficulty);

    let effort = difficulty * volume;
    println!("Effort:     {:.2}", effort);
}

fn main() {
    let mut i = 0;

    let bytecode: Vec<char> = read_bytecode().expect("Error reading bytecode");

    let first_opcode: String = bytecode[i..i + 2].iter().collect();
    let mut opcode = Opcode::from_byte(&first_opcode);

    i += 2;

    let mut stats = Stats::new(1, opcode.stack_input_size);

    while i < bytecode.len() - 1 {
        if opcode.operand_size > 0 {
            let data: String = bytecode[i..=i + (opcode.operand_size * 2) - 1]
                .iter()
                .collect();
            stats.add_operand(data.clone());
            opcode.operand = data;
            i += opcode.operand_size * 2;
            opcode.operand_size = 0;
        } else {
            stats.inc_opcode_count();
            let mut opcode_string = String::new();
            opcode_string.push(bytecode[i]);
            opcode_string.push(bytecode[i + 1]);

            opcode = Opcode::from_byte(&opcode_string);
            stats.inc_operand_count(opcode.stack_input_size);

            i += 2;
        }

        if opcode.has_operand && opcode.operand != "" {
            println!("{}", opcode);
        }
        if !opcode.has_operand {
            println!("{}", opcode);
        }
        stats.add_opcode(opcode.clone());
    }

    print_metrics(&stats);
}
