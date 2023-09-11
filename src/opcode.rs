use crate::operand::Operand;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Opcode {
    pub byte: String,
    pub name: String,
    pub operand: Operand,
    pub stack_input_size: usize,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<5} {}", self.name, self.operand.value)
    }
}

impl Opcode {
    pub fn new(byte: String) -> Opcode {
        Opcode {
            byte,
            name: "NOP".to_string(),
            operand: Operand::new(0),
            stack_input_size: 0,
        }
    }

    pub fn from_byte(byte: &String) -> Opcode {
        let mut opcode = Self::new(byte.to_string());
        match byte.as_str() {
            "00" => opcode.name = "STOP".to_string(),
            "01" => {
                opcode.name = "ADD".to_string();
                opcode.stack_input_size = 2;
            }
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
                opcode.stack_input_size = 1;
            }
            "1A" => {
                opcode.name = "BYTE".to_string();
                opcode.stack_input_size = 1;
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
            byte => {
                let byte = usize::from_str_radix(&byte, 16).unwrap();
                match byte {
                    0x5F..=0x7F => {
                        let operand_size = byte - 0x5F;
                        opcode.name = format!("PUSH{}", operand_size);
                        opcode.operand.size = operand_size;
                        opcode.stack_input_size = 0;
                    }
                    0x80..=0x8F => {
                        opcode.name = format!("DUP{}", byte - 0x7F);
                        opcode.stack_input_size = 2;
                    }
                    0x90..=0x9F => {
                        opcode.name = format!("SWAP{}", byte - 0x87);
                        opcode.stack_input_size = 2;
                    }
                    _ => (),
                }
            }
        }
        opcode
    }
}
