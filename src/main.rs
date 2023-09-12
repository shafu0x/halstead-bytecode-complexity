use std::env;

mod disassembler;
mod opcode;
mod operand;
mod stats;

use disassembler::Disassembler;

const METADATA_FLAG: &str = "--rm-metadata";
const VERBOSE_FLAG: &str = "--v";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path> [{}] [{}]", args[0], VERBOSE_FLAG, METADATA_FLAG);
        std::process::exit(1);
    }

    let path = &args[1];
    let remove_metadata = args.contains(&METADATA_FLAG.to_string());
    let verbose = args.contains(&VERBOSE_FLAG.to_string());

    let mut disassembler = Disassembler::new(path, remove_metadata);

    while let Ok(opcode) = disassembler.next_opcode() {
        if verbose {
            println!("{:>5}: {}", disassembler.line_number, opcode);
        }
    }

    disassembler.print_stats();
}
