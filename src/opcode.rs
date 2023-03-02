use std::fmt;

pub type Value = isize;
pub type Pointer = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Op {
    pub opcode: Opcode,
    pub data: Option<OpData>,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.opcode)?;

        if let Some(data) = &self.data {
            match data {
                OpData::Value(value) => write!(f, " {value} ({value:01x})"),
                OpData::Pointer(pointer) => write!(f, " {pointer}"),
            }
        } else {
            Ok(())
        }
    }
}

impl Op {
    pub fn new(opcode: Opcode, data: Option<OpData>) -> Self {
        Self { opcode, data }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Opcode {
    Noop = 0x00,
    Push = 0x01,
    Add = 0x02,
    Sub = 0x03,
    Mul = 0x04,
    Div = 0x05,
    Mod = 0x06,
    Jump = 0x07,
    JumpIfFalse = 0x08,
    Print = 0x09,
    PrintChar = 0x0A,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[0x{:01x}] ", unsafe {
            std::mem::transmute::<_, u8>(self.clone())
        })?;

        match self {
            Opcode::Noop => write!(f, "Noop"),
            Opcode::Push => write!(f, "Push"),
            Opcode::Add => write!(f, "Add"),
            Opcode::Sub => write!(f, "Sub"),
            Opcode::Mul => write!(f, "Mul"),
            Opcode::Div => write!(f, "Div"),
            Opcode::Mod => write!(f, "Mod"),
            Opcode::Jump => write!(f, "Jump"),
            Opcode::JumpIfFalse => write!(f, "JumpIfFalse"),
            Opcode::Print => write!(f, "Print"),
            Opcode::PrintChar => write!(f, "PrintChar"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpData {
    Value(Value),
    Pointer(Pointer),
}
