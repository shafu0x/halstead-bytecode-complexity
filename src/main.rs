use std::fs;
use std::env;
use std::fmt;

#[derive(Debug)]
struct Opcode {
    byte: String, 
    name: String, 
}

impl Opcode {
    fn new(byte: String, name: String) -> Opcode {
        Opcode { byte, name }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.byte, self.name)
    }
}

fn byte_to_opcode(byte: &String) -> Opcode {
    let mut opcode = Opcode::new(byte.to_string(), "NOP".to_string());
    match byte.as_str() {
        "60" => opcode.name = "PUSH1".to_string(),
        "80" => opcode.name = "DUP1".to_string(),
        _ => (),
    }
    opcode
}

fn main() {
    // get path to file from command line
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    // remove first two chars from contents
    let contents = &contents[2..];
    println!("File contents:\n{}", contents);

    let mut i = 0;
    let content_chars: Vec<char> = contents.chars().collect();
    while i < content_chars.len() {
        let mut s = String::new();
        s.push(content_chars[i]);
        s.push(content_chars[i+1]);
        i += 2;

        let opcode = byte_to_opcode(&s);
        println!("{}", opcode);
    }
}
