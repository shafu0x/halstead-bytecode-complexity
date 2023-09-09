use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Opcode {
    pub byte: String,
    pub name: String,
    pub operand_size: usize, // in bytes
    pub data: String,
    pub has_data: bool,
    pub stack_input_size: usize, 
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<5} {}", self.name, self.data)
    }
}

impl Opcode {
    pub fn new(byte: String) -> Opcode {
        Opcode {
            byte,
            name: "NOP".to_string(),
            operand_size: 0,
            data: "".to_string(),
            has_data: false,
            stack_input_size: 0,
        }
    }

    pub fn from_byte(&self, byte: &String) -> Opcode {
        let mut opcode = Self::new(byte.to_string());
        match byte.as_str() {
            "00" => opcode.name = "STOP".to_string(),
            s => ()
        }
        opcode
    }
}

