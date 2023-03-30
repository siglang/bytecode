use std::fmt;

pub type Value = isize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Op {
    pub opcode: Opcode,
    pub data: OpData,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.opcode, self.data)?;

        Ok(())
    }
}

impl Op {
    pub fn new(opcode: Opcode, data: OpData) -> Self {
        Self { opcode, data }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    Call = 0x0B,
    Arguments = 0x0C,
    Debug = 0xFF,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[0x{:x}] ", unsafe {
            std::mem::transmute::<_, u8>(*self)
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
            Opcode::Call => write!(f, "Call"),
            Opcode::Arguments => write!(f, "Arguments"),
            Opcode::Debug => write!(f, "Debug"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpData {
    /* 0x00 */ None,
    /* 0x01 */ Value(Value),
    /* 0x02 */ Pointer(usize),
    /* 0x03 */ Length(usize),
}

impl fmt::Display for OpData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let opdata = u8::from(*self);
        match self {
            OpData::None => write!(f, "{opdata}"),
            OpData::Value(value) => write!(f, "{opdata} 0x{value:x} ({value})"),
            OpData::Pointer(pointer) => write!(f, "{opdata} 0x{pointer:x} ({pointer})"),
            OpData::Length(length) => write!(f, "{opdata} 0x{length:x} ({length})"),
        }
    }
}

impl From<OpData> for u8 {
    fn from(op_data: OpData) -> Self {
        match op_data {
            OpData::None => 0x00,
            OpData::Value(_) => 0x01,
            OpData::Pointer(_) => 0x02,
            OpData::Length(_) => 0x03,
        }
    }
}
