use crate::{
    error::{BytecodeError, BytecodeResult},
    opcode::{Op, OpData, Opcode, Value},
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Program<'a>(pub &'a [Op<'a>]);

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

impl<'a> Vm<'a> {
    pub fn new(program: Program<'a>) -> Self {
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

                self.stack.push(second $op first);
            }};
        }

        macro_rules! prints {
            ($op:expr; $expr:expr) => {
                match $op.data {
                    Some([OpData::Length(lens), ..]) => {
                        for _ in 0..*lens {
                            $expr
                        }
                    },
                    _ => $expr,
                }
            };
        }

        while let Some(op) = self.program.0.get(pointer) {
            match op.opcode {
                Opcode::Noop => {}
                Opcode::Push => {
                    let values = match op.data {
                        Some(values) => values,
                        _ => return Err(BytecodeError::ParameterNotProvided),
                    };

                    match values {
                        [values @ ..] => {
                            for value in values {
                                match value {
                                    OpData::Value(value) => self.stack.push(*value),
                                    _ => return Err(BytecodeError::InvalidValue),
                                }
                            }
                        }
                    }
                }
                Opcode::Add => operator! { + },
                Opcode::Sub => operator! { - },
                Opcode::Mul => operator! { * },
                Opcode::Div => operator! { / },
                Opcode::Mod => operator! { % },
                Opcode::Jump => match op.data {
                    Some([OpData::Pointer(ptr), ..]) => {
                        pointer = *ptr;
                        continue;
                    }
                    _ => return Err(BytecodeError::PointerNotProvided),
                },
                Opcode::JumpIfFalse => match op.data {
                    Some([OpData::Pointer(ptr), ..]) => {
                        if self.stack.pop()? == 0 {
                            pointer = *ptr;
                            continue;
                        }
                    }
                    _ => return Err(BytecodeError::PointerNotProvided),
                },
                Opcode::Print => prints! {
                    op;
                    print!("{}", self.stack.pop()?)
                },
                Opcode::PrintChar => prints! {
                    op;
                    print!(
                        "{}",
                        std::char::from_u32(self.stack.pop()? as u32)
                            .ok_or(BytecodeError::InvalidValue)?
                    )
                },
            }

            pointer += 1;
        }

        Ok(())
    }
}
