use std::fs;


pub struct Lexer {
    pub bytecode: Vec<char>,
    index: usize,
}

fn read_bytecode(path: &String, with_metadata: bool) -> Vec<char> {
    let mut bytecode = fs::read_to_string(path).unwrap();
    
    // remove 0x if it exists
    if &bytecode[0..2] == "0x" {
        bytecode = bytecode[2..].to_string();
    }
    // remove metadata
    if !with_metadata {
        bytecode = strip_metadata(&bytecode);
    }

    bytecode.chars().collect()
}

// strip the metadata from the bytecode
fn strip_metadata(bytecode: &String) -> String {
    // metadata length is given by the last byte
    let last_byte: String = bytecode[bytecode.len() - 3..bytecode.len() - 1].to_string();
    let metadata_len = usize::from_str_radix(&last_byte, 16).unwrap();
    bytecode[0..bytecode.len() - (metadata_len * 2) - 4].to_string()
}

impl Lexer {
    pub fn new(path: &String, with_metadata: bool) -> Lexer {
        Lexer {
            bytecode: read_bytecode(path, with_metadata),
            index: 0, 
        }
    }

}
