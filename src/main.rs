use std::env;
use std::fs;

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
        eprintln!(
            "Usage: {} <directory> [{}] [{}]",
            args[0], VERBOSE_FLAG, METADATA_FLAG
        );
        std::process::exit(1);
    }

    let directory = &args[1];
    let remove_metadata = args.contains(&METADATA_FLAG.to_string());
    let verbose = args.contains(&VERBOSE_FLAG.to_string());

    match fs::read_dir(directory) {
        Ok(files) => {
            for file in files {
                let path = file.unwrap().path();
                let path_str = &path.to_str().unwrap().to_string();
                println!("{}", path_str);
                let mut disassembler = Disassembler::new(path_str, remove_metadata);

                while let Ok(opcode) = disassembler.next_opcode() {
                    if verbose {
                        println!("{:>5}: {}", disassembler.line_number, opcode);
                    }
                }
                disassembler.print_stats();
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
