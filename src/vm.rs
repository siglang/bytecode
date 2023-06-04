use crate::{
    error::{BytecodeError, BytecodeErrorKind},
    opcode::{Op, OpcodeV1},
    Pointer, Value,
};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, Default)]
pub struct Instructions<'a>(pub &'a [Op]);

#[derive(Debug, Clone, Copy, Default)]
pub struct Program<'a>(pub Instructions<'a>);

impl fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, op) in self.0 .0.iter().enumerate() {
            writeln!(f, "{index}: {op}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
struct Stack(pub Vec<Value>);

impl Stack {
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, BytecodeError> {
        self.0.pop().ok_or((BytecodeErrorKind::StackIsEmpty, None))
    }
}

#[derive(Debug, Default)]
pub struct Vm<'a> {
    pub program: Program<'a>,
    stack: Stack,
    call_stack: CallStack,
    store: Store,
}

#[derive(Debug, Clone, Copy)]
struct CallStackFrame {
    pointer: Pointer,
}

type CallStack = Vec<CallStackFrame>;

type Store = HashMap<usize, Value>;

enum OpExecuted {
    Ok,
    Continue,
    Break,
}

impl<'a> Vm<'a> {
    pub fn new(program: Program<'a>) -> Self {
        Vm {
            program,
            ..Default::default()
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

    fn get_operand(&self, op: &Op, pointer: Pointer) -> Result<Value, BytecodeError> {
        op.operand
            .ok_or((BytecodeErrorKind::ValueNotProvided, Some(pointer)))
    }

    fn execute_op(&mut self, op: &Op, pointer: &mut Pointer) -> Result<OpExecuted, BytecodeError> {
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
                self.stack.push(self.get_operand(op, *pointer)?);
            }
            OpcodeV1::Add => operator! { + },
            OpcodeV1::Sub => operator! { - },
            OpcodeV1::Mul => operator! { * },
            OpcodeV1::Div => operator! { / },
            OpcodeV1::Mod => operator! { % },
            OpcodeV1::Jump => {
                *pointer = self.get_operand(op, *pointer)? as Pointer;
                return Ok(OpExecuted::Continue);
            }
            OpcodeV1::JumpIfFalse => {
                if self.stack.pop()? == 0 {
                    *pointer = self.stack.pop()? as Pointer;
                    return Ok(OpExecuted::Continue);
                }
            }
            OpcodeV1::GT => inequality! { > },
            OpcodeV1::LT => inequality! { < },
            OpcodeV1::GTE => inequality! { >= },
            OpcodeV1::LTE => inequality! { <= },
            OpcodeV1::EQ => inequality! { == },
            OpcodeV1::Proc => {
                *pointer = *pointer + (self.get_operand(op, *pointer)? as Pointer) + 1 /* proc */;
                return Ok(OpExecuted::Continue);
            }
            OpcodeV1::Call => {
                self.call_stack.push(CallStackFrame {
                    pointer: *pointer + 1,
                });

                *pointer = self.get_operand(op, *pointer)? as Pointer;
                return Ok(OpExecuted::Continue);
            }
            OpcodeV1::Return => {
                let frame = self
                    .call_stack
                    .pop()
                    .ok_or((BytecodeErrorKind::CallStackIsEmpty, Some(*pointer)))?;
                *pointer = frame.pointer;

                return Ok(OpExecuted::Continue);
            }
            OpcodeV1::Store => {
                let value = self.stack.pop()?;
                let key = self.get_operand(op, *pointer)? as usize;

                self.store.insert(key, value);
            }
            OpcodeV1::Load => {
                let key = self.get_operand(op, *pointer)? as usize;
                let value = self
                    .store
                    .get(&key)
                    .ok_or((BytecodeErrorKind::StoreIndexNotFound(key), Some(*pointer)))?;

                self.stack.push(*value);
            }
            OpcodeV1::Exit => return Ok(OpExecuted::Break),
            OpcodeV1::Debug => {
                println!("[{}] Stack: {:?}", pointer, self.stack);

                let value = self.get_operand(op, *pointer)?;
                match value {
                    -1 => {
                        for _ in 0..self.stack.0.len() {
                            self.stack.pop()?;
                        }
                    }
                    _ => {
                        for _ in 0..value {
                            self.stack.pop()?;
                        }
                    }
                }
            }
        };

        Ok(OpExecuted::Ok)
    }
}
