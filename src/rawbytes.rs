use crate::{
    error::BytecodeError,
    opcode::{Op, OpcodeV1},
    vm::Instructions,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawBytes<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for RawBytes<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ByteCode<'a> {
    pub bytes: RawBytes<'a>,
}

impl<'a> From<&'a [u8]> for ByteCode<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }
}

impl<'a> TryFrom<RawBytes<'a>> for Instructions<'a> {
    type Error = BytecodeError;

    fn try_from(raw_bytes: RawBytes<'a>) -> Result<Self, Self::Error> {
        let mut instructions = Vec::new();

        let mut index = 0;
        let bytes = raw_bytes.0;

        while index < bytes.len() {
            let opcode = OpcodeV1::try_from(bytes[index])?;
            let operand = bytes
                .get(index + 1)
                .ok_or(BytecodeError::ValueKindNotProvided)?;

            match operand {
                // None
                0x00 => {
                    instructions.push(Op::new(opcode, None));
                    index += 1;
                }
                // Value
                0x01 => {
                    // Represent data using Little Endian.
                    // uses 1 byte per piece of data.
                    // for example, `1` is `[1, 0, 0, 0, 0, 0, 0, 0]`

                    let mut value_bytes = [0; 8];
                    value_bytes.copy_from_slice(&bytes[index + 2..index + 10]);

                    instructions.push(Op::new(opcode, Some(isize::from_le_bytes(value_bytes))));

                    index += 9;
                }
                opcode => {
                    return Err(BytecodeError::InvalidOpcode(*opcode));
                }
            }

            index += 1;
        }

        Ok(Self(Box::leak(instructions.into_boxed_slice())))
    }
}

impl<'a> From<Instructions<'a>> for RawBytes<'a> {
    fn from(instructions: Instructions<'a>) -> Self {
        let mut bytes = Vec::new();

        for op in instructions.0 {
            bytes.push(op.opcode as u8);

            match op.data {
                // None
                None => {
                    bytes.push(0x00);
                }
                // Value
                Some(value) => {
                    bytes.push(0x01);
                    bytes.append(&mut value.to_le_bytes().to_vec());
                }
            }
        }

        Self(Box::leak(bytes.into_boxed_slice()))
    }
}
