mod lexer;
mod opcode;
mod operand;
mod stats;

use std::env;

use lexer::Lexer;

const METADATA_FLAG: &str = "--rm-metadata";

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut lexer = Lexer::new(path, args.len() > 2 && &args[2] == METADATA_FLAG);

    while let Ok(opcode) = lexer.next_opcode() {
        println!("{}: {}", lexer.line_number, opcode);
    }

    lexer.stats.print();
}
