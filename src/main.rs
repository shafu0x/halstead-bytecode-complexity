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
    let mut disassembler = Disassembler::new(path, args.len() > 2 && &args[2] == METADATA_FLAG);

    while let Ok(opcode) = disassembler.next_opcode() {
        println!("{}: {}", disassembler.line_number, opcode);
    }

    disassembler.stats.print();
}
