use crate::{
    error::{BytecodeError, BytecodeErrorKind},
    Value,
};
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
    EQ = 0x0D,
    Proc = 0x0E,
    Call = 0x0F,
    Return = 0x10,
    Exit = 0xFE,
    Debug = 0xFF,
}

impl fmt::Display for OpcodeV1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use OpcodeV1::*;

        match self {
            Noop => write!(f, "Noop"),
            Push => write!(f, "Push"),
            Add => write!(f, "Add"),
            Sub => write!(f, "Sub"),
            Mul => write!(f, "Mul"),
            Div => write!(f, "Div"),
            Mod => write!(f, "Mod"),
            Jump => write!(f, "Jump"),
            JumpIfFalse => write!(f, "JumpIfFalse"),
            GT => write!(f, "GT"),
            LT => write!(f, "LT"),
            GTE => write!(f, "GTE"),
            LTE => write!(f, "LTE"),
            EQ => write!(f, "EQ"),
            Proc => write!(f, "Proc"),
            Call => write!(f, "Call"),
            Return => write!(f, "Return"),
            Exit => write!(f, "Exit"),
            Debug => write!(f, "Debug"),
        }
    }
}

impl TryFrom<u8> for OpcodeV1 {
    type Error = BytecodeError;

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        use OpcodeV1::*;

        Ok(match opcode {
            0x00 => Noop,
            0x01 => Push,
            0x02 => Add,
            0x03 => Sub,
            0x04 => Mul,
            0x05 => Div,
            0x06 => Mod,
            0x07 => Jump,
            0x08 => JumpIfFalse,
            0x09 => GT,
            0x0A => LT,
            0x0B => GTE,
            0x0C => LTE,
            0x0D => EQ,
            0x0E => Proc,
            0x0F => Call,
            0x10 => Return,
            0xFE => Exit,
            0xFF => Debug,
            _ => return Err((BytecodeErrorKind::InvalidOpcode(opcode), None)),
        })
    }
}

impl<'a> TryFrom<&'a str> for OpcodeV1 {
    type Error = BytecodeError;

    fn try_from(opcode: &'a str) -> Result<Self, Self::Error> {
        use OpcodeV1::*;

        Ok(match opcode {
            "noop" => Noop,
            "push" => Push,
            "add" => Add,
            "sub" => Sub,
            "mul" => Mul,
            "div" => Div,
            "mod" => Mod,
            "jump" => Jump,
            "jump_if_false" => JumpIfFalse,
            "gt" => GT,
            "lt" => LT,
            "gte" => GTE,
            "lte" => LTE,
            "eq" => EQ,
            "proc" => Proc,
            "call" => Call,
            "return" => Return,
            "exit" => Exit,
            "debug" => Debug,
            _ => return Err((BytecodeErrorKind::InvalidOpcode2(opcode.to_string()), None)),
        })
    }
}
