use std::collections::HashSet;
use std::env;
use std::fs;

mod opcode;
use opcode::Opcode;

const METADATA_FLAG: &str = "--rm-metadata";

// strip the metadata from the bytecode
fn strip_metadata(bytecode: &String) -> String {
    // metadata length is given by the last byte
    let last_byte: String = bytecode[bytecode.len() - 3..bytecode.len() - 1].to_string();
    let metadata_len = usize::from_str_radix(&last_byte, 16).unwrap();
    bytecode[0..bytecode.len() - (metadata_len * 2) - 4].to_string()
}

// read the bytecode from the file
fn read_bytecode() -> String {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut bytecode = fs::read_to_string(path).expect("Something went wrong reading the file");
    // remove 0x if it exists
    if &bytecode[0..2] == "0x" {
        bytecode = bytecode[2..].to_string();
    }
    // remove metadata
    if args.iter().any(|arg| arg == METADATA_FLAG) {
        bytecode = strip_metadata(&bytecode);
    }

    bytecode
}

fn get_number_of_unique_opcodes(opcodes: &Vec<Opcode>) -> usize {
    let mut unique_opcodes = HashSet::new();

    for opcode in opcodes {
        unique_opcodes.insert(&opcode.name);
    }
    unique_opcodes.len()
}

fn main() {
    let bytecode = read_bytecode();

    let mut i = 0;

    let content_chars: Vec<char> = bytecode.chars().collect();

    let mut first_opcode = String::new();
    first_opcode.push(content_chars[i]);
    first_opcode.push(content_chars[i + 1]);
    let mut opcode = Opcode::from_byte(&first_opcode);

    i += 2;

    let mut number_of_operations = 1;
    let mut number_of_operands = opcode.stack_input_size;

    let mut opcodes: Vec<Opcode> = Vec::new();
    let mut operands: Vec<String> = Vec::new();

    while i < content_chars.len() - 1 {
        if opcode.operand_size > 0 {
            let data: String = content_chars[i..=i + (opcode.operand_size * 2) - 1]
                .iter()
                .collect();
            operands.push(data.clone());
            opcode.operand = data;
            i += opcode.operand_size * 2;
            opcode.operand_size = 0;
        } else {
            number_of_operations += 1;
            let mut opcode_string = String::new();
            opcode_string.push(content_chars[i]);
            opcode_string.push(content_chars[i + 1]);

            opcode = Opcode::from_byte(&opcode_string);
            number_of_operands += opcode.stack_input_size;
            i += 2;
        }

        if opcode.has_operand && opcode.operand != "" {
            println!("{}", opcode);
        }
        if !opcode.has_operand {
            println!("{}", opcode);
        }
        opcodes.push(opcode.clone());
    }

    println!();
    let unique_opcodes = get_number_of_unique_opcodes(&opcodes);

    let mut unique_operands = HashSet::new();
    for op in operands {
        unique_operands.insert(op);
    }

    let vocabulary = unique_opcodes + unique_operands.len();
    println!("Vocabulary: {}", vocabulary);

    let length = number_of_operations + number_of_operands;
    println!("Length:     {}", length);

    let volume = length as f64 * (vocabulary as f64).log2();
    println!("Volume:     {:.2}", volume);

    let difficulty = (unique_opcodes as f64) / 2.0 * (number_of_operands as f64)
        / (unique_operands.len() as f64);
    println!("Difficulty: {:.2}", difficulty);

    let effort = difficulty * volume;
    println!("Effort:     {:.2}", effort);
}
