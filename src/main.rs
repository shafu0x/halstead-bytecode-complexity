use std::env;

mod disassembler;
mod opcode;
mod operand;
mod stats;

use disassembler::Disassembler;

const METADATA_FLAG: &str = "--rm-metadata";

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let strip_metadata = args.contains(&METADATA_FLAG.to_string());

    let mut disassembler = Disassembler::new(path, strip_metadata);

    while let Ok(opcode) = disassembler.next_opcode() {
        println!("{}: {}", disassembler.line_number, opcode);
    }

    disassembler.stats.print();
}
