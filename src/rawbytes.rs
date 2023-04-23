use crate::{
    error::BytecodeError,
    opcode::{Op, OpcodeV1},
    vm::Instructions,
    Value,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawBytes<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for RawBytes<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

impl<'a> TryFrom<RawBytes<'a>> for Instructions<'a> {
    type Error = BytecodeError;

    fn try_from(raw_bytes: RawBytes<'a>) -> Result<Self, Self::Error> {
        let mut instructions = Vec::new();

        let mut index = 0;
        let bytes = raw_bytes.0;

        while index < bytes.len() {
            use OpcodeV1::*;

            let opcode = OpcodeV1::try_from(bytes[index])?;

            match opcode {
                Push | Jump | JumpIfFalse | Debug => {
                    let mut value_bytes = [0; 8];
                    value_bytes.copy_from_slice(&bytes[index + 1..index + 9]);

                    instructions.push(Op::new(opcode, Some(Value::from_le_bytes(value_bytes))));

                    index += 9;
                }
                _ => {
                    instructions.push(Op::new(opcode, None));
                    index += 1;
                }
            }
        }

        Ok(Self(Box::leak(instructions.into_boxed_slice())))
    }
}

impl<'a> From<Instructions<'a>> for RawBytes<'a> {
    fn from(instructions: Instructions<'a>) -> Self {
        let mut bytes = Vec::new();

        for op in instructions.0 {
            bytes.push(op.opcode as u8);

            if let Some(value) = op.operand {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
        }

        Self(Box::leak(bytes.into_boxed_slice()))
    }
}
