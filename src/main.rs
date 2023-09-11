mod lexer;
mod opcode;
mod operand;
mod stats;

use std::env;
use std::fs;
use std::io;

use lexer::Lexer;
use opcode::Opcode;
use operand::Operand;
use stats::Stats;

const METADATA_FLAG: &str = "--rm-metadata";

// strip the metadata from the bytecode
fn strip_metadata(bytecode: &String) -> String {
    // metadata length is given by the last byte
    let last_byte: String = bytecode[bytecode.len() - 3..bytecode.len() - 1].to_string();
    let metadata_len = usize::from_str_radix(&last_byte, 16).unwrap();
    bytecode[0..bytecode.len() - (metadata_len * 2) - 4].to_string()
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

    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut lexer = Lexer::new(path, args.len() > 2 && &args[2] == METADATA_FLAG);

    for i in 1..5000 {
        match lexer.next_opcode() {
            Ok(opcode) => println!("{}: {}", i, opcode),
            Err(e) => break,
        }
    }

    // print_metrics(&stats);
}
