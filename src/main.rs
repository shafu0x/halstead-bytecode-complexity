use std::env;
use std::fmt;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Opcode {
    byte: String,
    name: String,
    operand_size: usize, // in bytes
    data: String,
    has_data: bool,
    stack_input_size: usize, 
}

impl Opcode {
    fn new(byte: String) -> Opcode {
        Opcode {
            byte,
            name: "NOP".to_string(),
            operand_size: 0,
            data: "".to_string(),
            has_data: false,
            stack_input_size: 0,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<5} {}", self.name, self.data)
    }
}

fn byte_to_push(opcode: &mut Opcode) -> &mut Opcode {
    let dec = usize::from_str_radix(&opcode.byte, 16).unwrap();
    let pushes = dec - 0x5F;
    opcode.operand_size = pushes;
    opcode.name = "PUSH".to_owned() + &pushes.to_string();
    opcode.has_data = true;
    opcode
}

fn byte_to_dup(opcode: &mut Opcode) -> &mut Opcode {
    let dec = usize::from_str_radix(&opcode.byte, 16).unwrap();
    let dups = dec - 0x7F;
    opcode.operand_size = dups;
    opcode.name = "DUP".to_owned() + &dups.to_string();
    opcode.stack_input_size = 1;
    opcode
}

fn byte_to_swap(opcode: &mut Opcode) -> &mut Opcode {
    let dec = usize::from_str_radix(&opcode.byte, 16).unwrap();
    let swaps = dec - 0x8F;
    opcode.operand_size = swaps;
    opcode.name = "SWAP".to_owned() + &swaps.to_string();
    opcode.stack_input_size = 1;
    opcode
}

fn byte_to_opcode(byte: &String) -> Opcode {
    let mut opcode = Opcode::new(byte.to_string());
    match byte.as_str() {
        "00" => opcode.name = "STOP".to_string(),
        "01" => {
            opcode.name = "ADD".to_string();
            opcode.stack_input_size = 2;
        },
        "02" => {
            opcode.name = "MUL".to_string();
            opcode.stack_input_size = 2;
        }
        "03" => {
            opcode.name = "SUB".to_string();
            opcode.stack_input_size = 2;
        }
        "04" => {
            opcode.name = "DIV".to_string();
            opcode.stack_input_size = 2;
        }
        "05" => {
            opcode.name = "SDIV".to_string();
            opcode.stack_input_size = 2;
        }
        "06" => {
            opcode.name = "MOD".to_string();
            opcode.stack_input_size = 2;
        }
        "07" => {
            opcode.name = "SMOD".to_string();
            opcode.stack_input_size = 2;
        }
        "08" => {
            opcode.name = "ADDMOD".to_string();
            opcode.stack_input_size = 3;
        }
        "09" => {
            opcode.name = "MULMOD".to_string();
            opcode.stack_input_size = 3;
        }
        "0A" => {
            opcode.name = "EXP".to_string();
            opcode.stack_input_size = 2;
        }
        "0B" => {
            opcode.name = "SIGNEXTEND".to_string();
            opcode.stack_input_size = 2;
        }
        "10" => {
            opcode.name = "LT".to_string();
            opcode.stack_input_size = 2;
        }
        "11" => {
            opcode.name = "GT".to_string();
            opcode.stack_input_size = 2;
        }
        "12" => {
            opcode.name = "SLT".to_string();
            opcode.stack_input_size = 2;
        }
        "13" => {
            opcode.name = "SGT".to_string();
            opcode.stack_input_size = 2;
        }
        "14" => {
            opcode.name = "EQ".to_string();
            opcode.stack_input_size = 2;
        }
        "15" => {
            opcode.name = "ISZERO".to_string();
            opcode.stack_input_size = 1;
        }
        "16" => {
            opcode.name = "AND".to_string();
            opcode.stack_input_size = 2;
        }
        "17" => {
            opcode.name = "OR".to_string();
            opcode.stack_input_size = 2;
        }
        "18" => {
            opcode.name = "XOR".to_string();
            opcode.stack_input_size = 2;
        }
        "19" => {
            opcode.name = "NOT".to_string();
            opcode.operand_size = 1;
        },
        "1A" => {
            opcode.name = "BYTE".to_string();
            opcode.operand_size = 1;
        }
        "1B" => {
            opcode.name = "SHL".to_string();
            opcode.stack_input_size = 2;
        }
        "1C" => {
            opcode.name = "SHR".to_string();
            opcode.stack_input_size = 2;
        }
        "1D" => {
            opcode.name = "SAR".to_string();
            opcode.stack_input_size = 2;
        }
        "20" => {
            opcode.name = "SHA3".to_string();
            opcode.stack_input_size = 2;
        }
        "30" => opcode.name = "ADDRESS".to_string(), 
        "31" => {
            opcode.name = "BALANCE".to_string();
            opcode.stack_input_size = 1;
        }
        "32" => opcode.name = "ORIGIN".to_string(),
        "33" => opcode.name = "CALLER".to_string(),
        "34" => opcode.name = "CALLVALUE".to_string(),
        "35" => {
            opcode.name = "CALLDATALOAD".to_string();
            opcode.stack_input_size = 1;
        }
        "36" => opcode.name = "CALLDATASIZE".to_string(),
        "37" => {
            opcode.name = "CALLDATACOPY".to_string();
            opcode.stack_input_size = 3;
        }
        "38" => opcode.name = "CODESIZE".to_string(),
        "39" => {
            opcode.name = "CODECOPY".to_string();
            opcode.stack_input_size = 3;
        }
        "3A" => opcode.name = "GASPRICE".to_string(),
        "3B" => {
            opcode.name = "EXTCODESIZE".to_string();
            opcode.stack_input_size = 1;
        }
        "3C" => {
            opcode.name = "EXTCODECOPY".to_string();
            opcode.stack_input_size = 4;
        }
        "3D" => opcode.name = "RETURNDATASIZE".to_string(),
        "3E" => {
            opcode.name = "RETURNDATACOPY".to_string();
            opcode.stack_input_size = 3;
        }
        "3F" => {
            opcode.name = "EXTCODEHASH".to_string();
            opcode.stack_input_size = 1;
        }
        "40" => {
            opcode.name = "BLOCKHASH".to_string();
            opcode.stack_input_size = 1;
        }
        "41" => opcode.name = "COINBASE".to_string(),
        "42" => opcode.name = "TIMESTAMP".to_string(),
        "43" => opcode.name = "NUMBER".to_string(),
        "44" => opcode.name = "PREVRANDAO".to_string(),
        "45" => opcode.name = "GASLIMIT".to_string(),
        "46" => opcode.name = "CHAINID".to_string(),
        "47" => opcode.name = "SELFBALANCE".to_string(),
        "48" => opcode.name = "BASEFEE".to_string(),
        "50" => {
            opcode.name = "POP".to_string();
            opcode.stack_input_size = 1;
        }
        "51" => {
            opcode.name = "MLOAD".to_string();
            opcode.stack_input_size = 1;
        }
        "52" => {
            opcode.name = "MSTORE".to_string();
            opcode.stack_input_size = 2;
        }
        "53" => {
            opcode.name = "MSTORE8".to_string();
            opcode.stack_input_size = 2;
        }
        "54" => {
            opcode.name = "SLOAD".to_string();
            opcode.stack_input_size = 1;
        }
        "55" => {
            opcode.name = "SSTORE".to_string();
            opcode.stack_input_size = 2;
        }
        "56" => {
            opcode.name = "JUMP".to_string();
            opcode.stack_input_size = 1;
        }
        "57" => {
            opcode.name = "JUMPI".to_string();
            opcode.stack_input_size = 2;
        }
        "58" => opcode.name = "PC".to_string(),
        "59" => opcode.name = "MSIZE".to_string(),
        "5A" => opcode.name = "GAS".to_string(),
        "5B" => opcode.name = "JUMPDEST".to_string(),
        "A0" => {
            opcode.name = "LOG0".to_string();
            opcode.stack_input_size = 2;
        }
        "A1" => {
            opcode.name = "LOG1".to_string();
            opcode.stack_input_size = 3;
        }
        "A2" => {
            opcode.name = "LOG2".to_string();
            opcode.stack_input_size = 4;
        }
        "A3" => {
            opcode.name = "LOG3".to_string();
            opcode.stack_input_size = 5;
        }
        "A4" => {
            opcode.name = "LOG4".to_string();
            opcode.stack_input_size = 6;
        }
        "F0" => {
            opcode.name = "CREATE".to_string();
            opcode.stack_input_size = 3;
        }
        "F1" => {
            opcode.name = "CALL".to_string();
            opcode.stack_input_size = 7;
        }
        "F2" => {
            opcode.name = "CALLCODE".to_string();
            opcode.stack_input_size = 7;
        }
        "F3" => {
            opcode.name = "RETURN".to_string();
            opcode.stack_input_size = 2;
        }
        "F4" => {
            opcode.name = "DELEGATECALL".to_string();
            opcode.stack_input_size = 6;
        }
        "F5" => {
            opcode.name = "CREATE2".to_string();
            opcode.stack_input_size = 4;
        }
        "FA" => {
            opcode.name = "STATICCALL".to_string();
            opcode.stack_input_size = 6;
        }
        "FD" => {
            opcode.name = "REVERT".to_string();
            opcode.stack_input_size = 2;
        }
        "FE" => opcode.name = "INVALID".to_string(),
        "FF" => {
            opcode.name = "SELFDESTRUCT".to_string();
            opcode.stack_input_size = 1;
        }
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

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    if &contents[0..2] == "0x" { 
        return contents[2..].to_string();
    } else {
        return contents.to_string();
    }
}

fn get_number_of_unique_opcodes(opcodes: &Vec<Opcode>) -> usize {
    let mut unique_opcodes = HashSet::new();

    for opcode in opcodes {
        unique_opcodes.insert(&opcode.name);
    }
    let count = unique_opcodes.len();
    count
}

fn main() {
    let contents = read_file();
    println!("Bytecode: {}", contents);

    let mut i = 0;

    let content_chars: Vec<char> = contents.chars().collect();

    let mut first_opcode = String::new();
    first_opcode.push(content_chars[i]);
    first_opcode.push(content_chars[i + 1]);
    let mut opcode = byte_to_opcode(&first_opcode);

    i += 2;

    let mut number_of_operations = 1;
    let mut number_of_operands = opcode.stack_input_size;

    let mut opcodes: Vec<Opcode> = Vec::new();

    while i < content_chars.len()-1 {
        if opcode.operand_size > 0 {
            let data: String = content_chars[i..=i + (opcode.operand_size * 2) - 1]
                .iter()
                .collect();
            opcode.data = data;
            i += opcode.operand_size * 2;
            opcode.operand_size = 0;
        } else {
            number_of_operations += 1;
            let mut opcode_string = String::new();
            opcode_string.push(content_chars[i]);
            opcode_string.push(content_chars[i + 1]);

            opcode = byte_to_opcode(&opcode_string);
            number_of_operands += opcode.stack_input_size;
            i += 2;
        }

        if opcode.has_data && opcode.data != "" {
            println!("{}", opcode);
        }
        if !opcode.has_data {
            println!("{}", opcode);
        }
        opcodes.push(opcode.clone());
    }

    println!("Number of operations: {}", number_of_operations);
    println!("Number of operands: {}", number_of_operands);

    // let mut unique_opcodes = HashSet::new();

    // for opcode in opcodes {
    //     unique_opcodes.insert(opcode.name);
    // }
    // let count = unique_opcodes.len();
    // println!("Number of unique elements: {}", count);

    let count = get_number_of_unique_opcodes(&opcodes);
    println!("Number of unique elements: {}", count);
}
