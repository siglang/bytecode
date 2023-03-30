use crate::{
    error::{BytecodeError, BytecodeResult},
    opcode::{Op, OpData, Opcode, Value},
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metadata<'a> {
    pub name: &'a str,
    pub id: usize,
    pub parameters: &'a [Value],
    // pub version: u8, // TODO
}

impl<'a> Metadata<'a> {
    pub fn new(name: &'a str, id: usize, parameters: &'a [Value]) -> Self {
        Self {
            name,
            id,
            parameters,
        }
    }
}

impl fmt::Display for Metadata<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "name: {} | id: {} | parameters: {:?}",
            self.name, self.id, self.parameters
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Procedure<'a> {
    pub metadata: Metadata<'a>,
    pub instructions: &'a [Op],
    pub procedures: &'a [Procedure<'a>],
}

impl<'a> Procedure<'a> {
    pub fn new(
        metadata: Metadata<'a>,
        instructions: &'a [Op],
        procedures: &'a [Procedure],
    ) -> BytecodeResult<Self> {
        if instructions.is_empty() {
            return Err(BytecodeError::ProgramIsEmpty);
        }

        Ok(Self {
            metadata,
            instructions,
            procedures,
        })
    }
}

impl fmt::Display for Procedure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, op) in self.instructions.iter().enumerate() {
            writeln!(f, "{index}: {op}")?;
        }

        writeln!(f, "[Info] {}", self.metadata)?;

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
    pub program: Procedure<'a>,
    pub stack: Stack,
}

enum OpExecuted {
    Ok,
    Continue,
}

impl<'a> Vm<'a> {
    pub fn new(program: Procedure<'a>) -> Self {
        Vm {
            program,
            stack: Stack::default(),
        }
    }

    pub fn run(&mut self) -> BytecodeResult<()> {
        let mut pointer = 0;

        while let Some(op) = self.program.instructions.get(pointer) {
            match self.execute_op(op, &mut pointer)? {
                OpExecuted::Ok => {}
                OpExecuted::Continue => {
                    continue;
                }
            }

            pointer += 1;
        }

        Ok(())
    }

    fn execute_op(&mut self, op: &Op, pointer: &mut usize) -> BytecodeResult<OpExecuted> {
        macro_rules! operator {
            ($op:tt) => {{
                let first = self.stack.pop()?;
                let second = self.stack.pop()?;

                self.stack.push(second $op first);
            }};
        }

        match op.opcode {
            Opcode::Noop => {}
            Opcode::Push => {
                let value = match op.data {
                    OpData::Value(value) => value,
                    _ => return Err(BytecodeError::ValueNotProvided),
                };

                self.stack.push(value);
            }
            Opcode::Add => operator! { + },
            Opcode::Sub => operator! { - },
            Opcode::Mul => operator! { * },
            Opcode::Div => operator! { / },
            Opcode::Mod => operator! { % },
            Opcode::Jump => match op.data {
                OpData::Pointer(ptr) => {
                    *pointer = ptr;
                    return Ok(OpExecuted::Continue);
                }
                _ => return Err(BytecodeError::PointerNotProvided),
            },
            Opcode::JumpIfFalse => match op.data {
                OpData::Pointer(ptr) => {
                    if self.stack.pop()? == 0 {
                        *pointer = ptr;
                        return Ok(OpExecuted::Continue);
                    }
                }
                _ => return Err(BytecodeError::PointerNotProvided),
            },
            Opcode::Print | Opcode::PrintChar => panic!("Deprecated"),
            Opcode::Call => todo!("Call"),
            Opcode::Arguments => {
                todo!();
                #[allow(unreachable_code)]
                // self.stack.push(self.program.metadata.parameters.len() as Value); // TODO
                for parameter in self.program.metadata.parameters {
                    self.stack.push(*parameter);
                }
            }
            Opcode::Debug => {
                println!(
                    "[{} ({})] Debug on pointer {} | Stack: {:?}",
                    self.program.metadata.name, self.program.metadata.id, pointer, self.stack
                );

                match op.data {
                    OpData::Value(value) => {
                        for _ in 0..value {
                            self.stack.pop()?;
                        }
                    }
                    _ => {
                        for _ in 0..self.stack.0.len() {
                            self.stack.pop()?;
                        }
                    }
                }
            }
        };

        Ok(OpExecuted::Ok)
    }
}

// #[derive(Debug, Copy, Clone, PartialEq)]
// struct ByteCode<'a> {
//     pub bytes: &'a [u8],
// }

// impl<'a> From<&'a [u8]> for ByteCode<'a> {
//     fn from(bytes: &'a [u8]) -> Self {
//         Self { bytes }
//     }
// }
