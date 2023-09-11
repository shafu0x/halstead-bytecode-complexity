use std::env;

mod disassembler;
mod opcode;
mod operand;
mod stats;

use disassembler::Disassembler;

const METADATA_FLAG: &str = "--rm-metadata";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path> [--strip-metadata]", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let strip_metadata = args.contains(&METADATA_FLAG.to_string());

    let mut disassembler = Disassembler::new(path, strip_metadata);

    while let Ok(opcode) = disassembler.next_opcode() {
        println!("{:>5}: {}", disassembler.line_number, opcode);
    }

    disassembler.print_stats();
}
