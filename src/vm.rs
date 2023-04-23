use crate::{
    error::{BytecodeError, BytecodeErrorKind},
    opcode::{Op, OpcodeV1},
    Pointer, Value,
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instructions<'a>(pub &'a [Op]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Program<'a>(pub Instructions<'a>);

impl fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, op) in self.0 .0.iter().enumerate() {
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

    pub fn pop(&mut self) -> Result<Value, BytecodeError> {
        self.0.pop().ok_or((BytecodeErrorKind::StackIsEmpty, None))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vm<'a> {
    pub program: Program<'a>,
    pub stack: Stack,
    pub call_stack: CallStack,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackFrame {
    pub pointer: Pointer,
}

pub type CallStack = Vec<StackFrame>;

enum OpExecuted {
    Ok,
    Continue,
    Break,
}

impl<'a> Vm<'a> {
    pub fn new(program: Program<'a>) -> Self {
        Vm {
            program,
            stack: Stack::default(),
            call_stack: CallStack::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), BytecodeError> {
        let mut pointer = 0;

        while let Some(op) = self.program.0 .0.get(pointer) {
            match self.execute_op(op, &mut pointer)? {
                OpExecuted::Ok => {}
                OpExecuted::Continue => {
                    continue;
                }
                OpExecuted::Break => {
                    break;
                }
            }

            pointer += 1;
        }

        Ok(())
    }

    fn execute_op(&mut self, op: &Op, pointer: &mut Pointer) -> Result<OpExecuted, BytecodeError> {
        println!("Executing op: {:?}", op);
        macro_rules! operator {
            ($op:tt) => {{
                let first = self.stack.pop()?;
                let second = self.stack.pop()?;

                self.stack.push(second $op first);
            }};
        }

        macro_rules! inequality {
            ($op:tt) => {{
                let first = self.stack.pop()?;
                let second = self.stack.pop()?;

                self.stack.push(if second $op first { 1 } else { 0 });
            }};
        }

        match op.opcode {
            OpcodeV1::Noop => {}
            OpcodeV1::Push => {
                if let Some(value) = op.operand {
                    self.stack.push(value);
                } else {
                    return Err((BytecodeErrorKind::ValueNotProvided, Some(*pointer)));
                }
            }
            OpcodeV1::Add => operator! { + },
            OpcodeV1::Sub => operator! { - },
            OpcodeV1::Mul => operator! { * },
            OpcodeV1::Div => operator! { / },
            OpcodeV1::Mod => operator! { % },
            OpcodeV1::Jump => match op.operand {
                Some(value) => {
                    *pointer = value as Pointer;
                    return Ok(OpExecuted::Continue);
                }
                None => return Err((BytecodeErrorKind::ValueNotProvided, Some(*pointer))),
            },
            OpcodeV1::JumpIfFalse => match op.operand {
                Some(value) => {
                    let condition = self.stack.pop()?;
                    if condition == 0 {
                        *pointer = value as Pointer;
                        return Ok(OpExecuted::Continue);
                    }
                }
                None => return Err((BytecodeErrorKind::ValueNotProvided, Some(*pointer))),
            },
            OpcodeV1::GT => inequality! { > },
            OpcodeV1::LT => inequality! { < },
            OpcodeV1::GTE => inequality! { >= },
            OpcodeV1::LTE => inequality! { <= },
            OpcodeV1::EQ => inequality! { == },
            // `proc [procedure instructions length]`
            //
            // All instructions in the procedure are ignored.
            // cannot use the procedure until it is called.
            //
            // proc 4
            //    push 1
            //    push 2
            //    mul
            //    return
            OpcodeV1::Proc => match op.operand {
                Some(value) => {
                    *pointer = *pointer + (value as Pointer) + 1 /* proc */;
                    return Ok(OpExecuted::Continue);
                }
                None => return Err((BytecodeErrorKind::ValueNotProvided, Some(*pointer))),
            },
            OpcodeV1::Call => match op.operand {
                Some(value) => {
                    self.call_stack.push(StackFrame {
                        pointer: *pointer + 1,
                    });

                    *pointer = value as Pointer;

                    return Ok(OpExecuted::Continue);
                }
                None => return Err((BytecodeErrorKind::ValueNotProvided, Some(*pointer))),
            },
            OpcodeV1::Return => {
                let frame = self
                    .call_stack
                    .pop()
                    .ok_or((BytecodeErrorKind::CallStackIsEmpty, Some(*pointer)))?;
                *pointer = frame.pointer;

                return Ok(OpExecuted::Continue);
            }
            OpcodeV1::Exit => return Ok(OpExecuted::Break),
            OpcodeV1::Debug => {
                println!("[{}] Stack: {:?}", pointer, self.stack);

                match op.operand {
                    Some(value) => match value {
                        0 => {
                            for _ in 0..value {
                                self.stack.pop()?;
                            }
                        }
                        1 => {
                            for _ in 0..self.stack.0.len() {
                                self.stack.pop()?;
                            }
                        }
                        _ => {}
                    },
                    None => return Err((BytecodeErrorKind::ValueNotProvided, Some(*pointer))),
                }
            }
        };

        Ok(OpExecuted::Ok)
    }
}
