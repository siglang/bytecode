use crate::{
    error::{BytecodeError, BytecodeResult},
    opcode::{Op, OpData, Opcode, Value},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a>(pub &'a [Op]);

impl fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, op) in self.0.iter().enumerate() {
            writeln!(f, "{index}: {op}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stack(pub Vec<Value>);

impl Stack {
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> BytecodeResult<Value> {
        self.0.pop().ok_or(BytecodeError::StackIsEmpty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vm<'a> {
    pub program: Program<'a>,
    pub stack: Stack,
}

impl Vm<'_> {
    pub fn new(program: Program) -> Vm {
        Vm {
            program,
            stack: Stack::default(),
        }
    }

    pub fn run(&mut self) -> BytecodeResult<()> {
        let mut pointer = 0;

        macro_rules! operator {
            ($op:tt) => {{
                let first = self.stack.pop()?;
                let second = self.stack.pop()?;

                self.stack.push(first $op second);
            }};
        }

        while let Some(op) = self.program.0.get(pointer) {
            match op.opcode {
                Opcode::Noop => {}
                Opcode::Push => {
                    if let Some(OpData::Value(value)) = op.data {
                        self.stack.push(value);
                    } else {
                        return Err(BytecodeError::ValueNotProvided);
                    }
                }
                Opcode::Add => operator! { + },
                Opcode::Sub => operator! { - },
                Opcode::Mul => operator! { * },
                Opcode::Div => operator! { / },
                Opcode::Mod => operator! { % },
                Opcode::Jump => {
                    if let Some(OpData::Pointer(ptr)) = op.data {
                        pointer = ptr;
                        continue;
                    } else {
                        return Err(BytecodeError::PointerNotProvided);
                    }
                }
                Opcode::JumpIfFalse => {
                    if let Some(OpData::Pointer(ptr)) = op.data {
                        if self.stack.pop()? == 0 {
                            pointer = ptr;
                            continue;
                        }
                    } else {
                        return Err(BytecodeError::PointerNotProvided);
                    }
                }
                Opcode::Print => print!("{}", self.stack.pop()?),
                Opcode::PrintChar => print!(
                    "{}",
                    std::char::from_u32(self.stack.pop()? as u32)
                        .ok_or(BytecodeError::InvalidValue)?
                ),
            }

            pointer += 1;
        }

        Ok(())
    }
}
