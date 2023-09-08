use std::fs;
use std::env;
use std::fmt;

#[derive(Debug)]
struct Opcode {
    byte: String, 
    name: String, 
    operand_size: usize, // in bytes
}

impl Opcode {
    fn new(byte: String) -> Opcode {
        Opcode { byte, name: "NOP".to_string(), operand_size: 0 }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.byte, self.name, self.operand_size)
    }
}

fn byte_to_push(opcode: &mut Opcode) -> &mut Opcode {
    let dec = usize::from_str_radix(&opcode.byte, 16).unwrap();
    let pushes = dec - 0x5F;
    opcode.operand_size = pushes;
    opcode.name = "PUSH".to_owned() + &pushes.to_string();
    opcode
}

fn byte_to_dup(opcode: &mut Opcode) -> &mut Opcode {
    let dec = usize::from_str_radix(&opcode.byte, 16).unwrap();
    let dups = dec - 0x7F;
    opcode.operand_size = dups;
    opcode.name = "DUP".to_owned() + &dups.to_string();
    opcode
}

fn byte_to_swap(opcode: &mut Opcode) -> &mut Opcode {
    let dec = usize::from_str_radix(&opcode.byte, 16).unwrap();
    let swaps = dec - 0x8F;
    opcode.operand_size = swaps;
    opcode.name = "SWAP".to_owned() + &swaps.to_string();
    opcode
}

fn byte_to_opcode(byte: &String) -> Opcode {
    let mut opcode = Opcode::new(byte.to_string());
    match byte.as_str() {
        "00" => opcode.name = "STOP".to_string(),
        "01" => opcode.name = "ADD".to_string(),
        "02" => opcode.name = "MUL".to_string(),
        "03" => opcode.name = "SUB".to_string(),
        s => {
            let dec = usize::from_str_radix(&s, 16).unwrap();
            if dec >= 0x5F && dec <= 0x7F {
                byte_to_push(&mut opcode);
            } 
            if dec >= 0x80 && dec <= 0x8F {
                byte_to_dup(&mut opcode);
            }
            if dec >= 0x90 && dec <= 0x9F {
                byte_to_swap(&mut opcode);
            }
        }
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
