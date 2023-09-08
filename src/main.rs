use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Opcode {
    byte: String,
    name: String,
    operand_size: usize, // in bytes
    data: String,
    has_data: bool,
}

impl Opcode {
    fn new(byte: String) -> Opcode {
        Opcode {
            byte,
            name: "NOP".to_string(),
            operand_size: 0,
            data: "".to_string(),
            has_data: false,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:<5} {}", self.byte, self.name, self.data)
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
    opcode.has_data = true;
    opcode
}

fn byte_to_swap(opcode: &mut Opcode) -> &mut Opcode {
    let dec = usize::from_str_radix(&opcode.byte, 16).unwrap();
    let swaps = dec - 0x8F;
    opcode.operand_size = swaps;
    opcode.name = "SWAP".to_owned() + &swaps.to_string();
    opcode.has_data = true;
    opcode
}

fn byte_to_opcode(byte: &String) -> Opcode {
    let mut opcode = Opcode::new(byte.to_string());
    match byte.as_str() {
        "00" => opcode.name = "STOP".to_string(),
        "01" => opcode.name = "ADD".to_string(),
        "02" => opcode.name = "MUL".to_string(),
        "03" => opcode.name = "SUB".to_string(),
        "04" => opcode.name = "DIV".to_string(),
        "05" => opcode.name = "SDIV".to_string(),
        "06" => opcode.name = "MOD".to_string(),
        "07" => opcode.name = "SMOD".to_string(),
        "08" => opcode.name = "ADDMOD".to_string(),
        "09" => opcode.name = "MULMOD".to_string(),
        "0A" => opcode.name = "EXP".to_string(),
        "0B" => opcode.name = "SIGNEXTEND".to_string(),
        "10" => opcode.name = "LT".to_string(),
        "11" => opcode.name = "GT".to_string(),
        "12" => opcode.name = "SLT".to_string(),
        "13" => opcode.name = "SGT".to_string(),
        "14" => opcode.name = "EQ".to_string(),
        "15" => opcode.name = "ISZERO".to_string(),
        "16" => opcode.name = "AND".to_string(),
        "17" => opcode.name = "OR".to_string(),
        "18" => opcode.name = "XOR".to_string(),
        "19" => opcode.name = "NOT".to_string(),
        "1A" => opcode.name = "BYTE".to_string(),
        "1B" => opcode.name = "SHL".to_string(),
        "1C" => opcode.name = "SHR".to_string(),
        "1D" => opcode.name = "SAR".to_string(),
        "20" => opcode.name = "SHA3".to_string(),
        "30" => opcode.name = "ADDRESS".to_string(),
        "31" => opcode.name = "BALANCE".to_string(),
        "32" => opcode.name = "ORIGIN".to_string(),
        "33" => opcode.name = "CALLER".to_string(),
        "34" => opcode.name = "CALLVALUE".to_string(),
        "35" => opcode.name = "CALLDATALOAD".to_string(),
        "36" => opcode.name = "CALLDATASIZE".to_string(),
        "37" => opcode.name = "CALLDATACOPY".to_string(),
        "38" => opcode.name = "CODESIZE".to_string(),
        "39" => opcode.name = "CODECOPY".to_string(),
        "3A" => opcode.name = "GASPRICE".to_string(),
        "3B" => opcode.name = "EXTCODESIZE".to_string(),
        "3C" => opcode.name = "EXTCODECOPY".to_string(),
        "3D" => opcode.name = "RETURNDATASIZE".to_string(),
        "3E" => opcode.name = "RETURNDATACOPY".to_string(),
        "3F" => opcode.name = "EXTCODEHASH".to_string(),
        "40" => opcode.name = "BLOCKHASH".to_string(),
        "41" => opcode.name = "COINBASE".to_string(),
        "42" => opcode.name = "TIMESTAMP".to_string(),
        "43" => opcode.name = "NUMBER".to_string(),
        "44" => opcode.name = "PREVRANDAO".to_string(),
        "45" => opcode.name = "GASLIMIT".to_string(),
        "46" => opcode.name = "CHAINID".to_string(),
        "47" => opcode.name = "SELFBALANCE".to_string(),
        "48" => opcode.name = "BASEFEE".to_string(),
        "50" => opcode.name = "POP".to_string(),
        "51" => opcode.name = "MLOAD".to_string(),
        "52" => opcode.name = "MSTORE".to_string(),
        "53" => opcode.name = "MSTORE8".to_string(),
        "54" => opcode.name = "SLOAD".to_string(),
        "55" => opcode.name = "SSTORE".to_string(),
        "56" => opcode.name = "JUMP".to_string(),
        "57" => opcode.name = "JUMPI".to_string(),
        "58" => opcode.name = "PC".to_string(),
        "59" => opcode.name = "MSIZE".to_string(),
        "5A" => opcode.name = "GAS".to_string(),
        "5B" => opcode.name = "JUMPDEST".to_string(),
        "A0" => opcode.name = "LOG0".to_string(),
        "A1" => opcode.name = "LOG1".to_string(),
        "A2" => opcode.name = "LOG2".to_string(),
        "A3" => opcode.name = "LOG3".to_string(),
        "A4" => opcode.name = "LOG4".to_string(),
        "F0" => opcode.name = "CREATE".to_string(),
        "F1" => opcode.name = "CALL".to_string(),
        "F2" => opcode.name = "CALLCODE".to_string(),
        "F3" => opcode.name = "RETURN".to_string(),
        "F4" => opcode.name = "DELEGATECALL".to_string(),
        "F5" => opcode.name = "CREATE2".to_string(),
        "FA" => opcode.name = "STATICCALL".to_string(),
        "FD" => opcode.name = "REVERT".to_string(),
        "FE" => opcode.name = "INVALID".to_string(),
        "FF" => opcode.name = "SELFDESTRUCT".to_string(),
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
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let contents = &contents[2..];
    println!("Bytecode: {}", contents);

    let mut i = 0;

    let content_chars: Vec<char> = contents.chars().collect();

    let mut s = String::new();
    s.push(content_chars[i]);
    s.push(content_chars[i + 1]);
    let mut opcode = byte_to_opcode(&s);

    i += 2;

    while i < content_chars.len()-1 {
        if opcode.operand_size > 0 {
            let data: String = content_chars[i..=i + (opcode.operand_size * 2) - 1]
                .iter()
                .collect();
            opcode.data = data;
            i += opcode.operand_size * 2;
            opcode.operand_size = 0;
        } else {
            let mut ss = String::new();
            ss.push(content_chars[i]);
            ss.push(content_chars[i + 1]);

            opcode = byte_to_opcode(&ss);
            i += 2;
        }

        if opcode.has_data && opcode.data != "" {
            println!("{}", opcode);
        }
        if !opcode.has_data {
            println!("{}", opcode);
        }
    }
}
