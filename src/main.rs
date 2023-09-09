use std::env;
use std::fs;
use std::collections::HashSet;

mod opcode;
use opcode::Opcode;

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    if &contents[0..2] == "0x" { 
        return contents[2..].to_string();
    } else {
        return contents.to_string();
    }
}

fn get_number_of_unique_opcodes(opcodes: &Vec<Opcode>) -> usize {
    let mut unique_opcodes = HashSet::new();

    for opcode in opcodes {
        unique_opcodes.insert(&opcode.name);
    }
    unique_opcodes.len()
}

fn main() {
    let contents = read_file();
    // println!("Bytecode: {}", contents);

    let mut i = 0;

    let content_chars: Vec<char> = contents.chars().collect();

    let mut first_opcode = String::new();
    first_opcode.push(content_chars[i]);
    first_opcode.push(content_chars[i + 1]);
    let mut opcode = Opcode::from_byte(&first_opcode);

    i += 2;

    let mut number_of_operations = 1;
    let mut number_of_operands = opcode.stack_input_size;

    let mut opcodes: Vec<Opcode> = Vec::new();
    let mut operands: Vec<String> = Vec::new();

    while i < content_chars.len()-1 {
        if opcode.operand_size > 0 {
            let data: String = content_chars[i..=i + (opcode.operand_size * 2) - 1]
                .iter()
                .collect();
            operands.push(data.clone());
            opcode.data = data;
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

        if opcode.has_data && opcode.data != "" {
            println!("{}", opcode);
        }
        if !opcode.has_data {
            println!("{}", opcode);
        }
        opcodes.push(opcode.clone());
    }

    println!();
    println!("Number of operations: {}", number_of_operations);
    println!("Number of operands: {}", number_of_operands);

    let unique_opcodes = get_number_of_unique_opcodes(&opcodes);
    println!("Number of unique opcodes: {}", unique_opcodes);

    let mut unique_operands = HashSet::new();
    for op in operands {
        unique_operands.insert(op);
    }
    println!("Number of unique operands: {}", unique_operands.len());

    let vocabulary = unique_opcodes + unique_operands.len();
    println!("Vocabulary: {}", vocabulary);

    let length = number_of_operations + number_of_operands;
    println!("Length: {}", length);

    let estimated_program_length = (unique_operands.len() as f64) * (unique_operands.len() as f64).log2() + (unique_opcodes as f64) * (unique_opcodes as f64).log2();
    println!("Estimated program length: {}", estimated_program_length);

    let volume = length as f64 * (vocabulary as f64).log2();
    println!("Volume: {}", volume);

    let difficulty = (unique_opcodes as f64) / 2.0 * (number_of_operands as f64) / (unique_operands.len() as f64);
    println!("Difficulty: {}", difficulty);

    let effort = difficulty * volume;
    println!("Effort: {}", effort);
}
