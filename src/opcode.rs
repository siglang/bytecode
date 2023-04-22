use crate::{error::BytecodeError, Value};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Op {
    pub opcode: OpcodeV1,
    pub operand: Option<Value>,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.opcode, self.operand.unwrap_or(0))?;

        Ok(())
    }
}

impl Op {
    pub fn new(opcode: OpcodeV1, operand: Option<Value>) -> Self {
        Self { opcode, operand }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum OpcodeV1 {
    Noop = 0x00,
    Push = 0x01,
    Add = 0x02,
    Sub = 0x03,
    Mul = 0x04,
    Div = 0x05,
    Mod = 0x06,
    Jump = 0x07,
    JumpIfFalse = 0x08,
    GT = 0x09,
    LT = 0x0A,
    GTE = 0x0B,
    LTE = 0x0C,
    Exit = 0xFE,
    Debug = 0xFF,
}

impl fmt::Display for OpcodeV1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpcodeV1::Noop => write!(f, "Noop"),
            OpcodeV1::Push => write!(f, "Push"),
            OpcodeV1::Add => write!(f, "Add"),
            OpcodeV1::Sub => write!(f, "Sub"),
            OpcodeV1::Mul => write!(f, "Mul"),
            OpcodeV1::Div => write!(f, "Div"),
            OpcodeV1::Mod => write!(f, "Mod"),
            OpcodeV1::Jump => write!(f, "Jump"),
            OpcodeV1::JumpIfFalse => write!(f, "JumpIfFalse"),
            OpcodeV1::GT => write!(f, "GT"),
            OpcodeV1::LT => write!(f, "LT"),
            OpcodeV1::GTE => write!(f, "GTE"),
            OpcodeV1::LTE => write!(f, "LTE"),
            OpcodeV1::Exit => write!(f, "Exit"),
            OpcodeV1::Debug => write!(f, "Debug"),
        }
    }
}

impl TryFrom<u8> for OpcodeV1 {
    type Error = BytecodeError;

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        Ok(match opcode {
            0x00 => OpcodeV1::Noop,
            0x01 => OpcodeV1::Push,
            0x02 => OpcodeV1::Add,
            0x03 => OpcodeV1::Sub,
            0x04 => OpcodeV1::Mul,
            0x05 => OpcodeV1::Div,
            0x06 => OpcodeV1::Mod,
            0x07 => OpcodeV1::Jump,
            0x08 => OpcodeV1::JumpIfFalse,
            0x09 => OpcodeV1::GT,
            0x0A => OpcodeV1::LT,
            0x0B => OpcodeV1::GTE,
            0x0C => OpcodeV1::LTE,
            0xFE => OpcodeV1::Exit,
            0xFF => OpcodeV1::Debug,
            _ => return Err(BytecodeError::InvalidOpcode(opcode)),
        })
    }
}

impl<'a> From<&'a str> for OpcodeV1 {
    fn from(opcode: &'a str) -> Self {
        match opcode {
            "noop" => OpcodeV1::Noop,
            "push" => OpcodeV1::Push,
            "add" => OpcodeV1::Add,
            "sub" => OpcodeV1::Sub,
            "mul" => OpcodeV1::Mul,
            "div" => OpcodeV1::Div,
            "mod" => OpcodeV1::Mod,
            "jump" => OpcodeV1::Jump,
            "jump_if_false" => OpcodeV1::JumpIfFalse,
            "gt" => OpcodeV1::GT,
            "lt" => OpcodeV1::LT,
            "gte" => OpcodeV1::GTE,
            "lte" => OpcodeV1::LTE,
            "exit" => OpcodeV1::Exit,
            "debug" => OpcodeV1::Debug,
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }
}
