use std::env;
use std::fs;

mod disassembler;
mod opcode;
mod operand;
mod stats;

use disassembler::run_dis;

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

    let dir = &args[1];
    let remove_metadata = args.contains(&METADATA_FLAG.to_string());
    let verbose = args.contains(&VERBOSE_FLAG.to_string());

    match fs::read_dir(dir) {
        Ok(files) => {
            for file in files {
                let path = file.unwrap().path();
                if path.is_file() {
                    run_dis(&path.to_str().unwrap(), remove_metadata, verbose);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
