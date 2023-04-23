pub mod error;
pub mod opcode;
pub mod rawbytes;
pub mod vm;

pub type Value = isize;
pub type Pointer = usize;

#[derive(Debug, Clone)]
pub struct Code<'a>(pub &'a str);

impl<'a> Code<'a> {
    pub fn parse(&self) -> vm::Instructions<'a> {
        let mut instructions = Vec::new();

        for line in self.0.trim().lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let split_whitespace = line.split_whitespace().collect::<Vec<_>>();

            instructions.push(Code::parse_op(split_whitespace));
        }

        vm::Instructions(Box::leak(instructions.into_boxed_slice()))
    }

    fn parse_op(op: Vec<&str>) -> opcode::Op {
        let opcode = op[0];

        match opcode.to_lowercase().as_str() {
            "jump" | "jump_if_false" | "debug" | "proc" | "call" => {
                let operand = op.get(1).map(|s| {
                    (if s.starts_with("0x") {
                        Pointer::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
                    } else {
                        s.parse::<Pointer>().unwrap()
                    }) as Value
                });
                opcode::Op::new(opcode::OpcodeV1::try_from(opcode).unwrap(), operand)
            }
            "push" => {
                let operand = op.get(1).map(|s| {
                    if s.starts_with("0x") {
                        Value::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
                    } else {
                        s.parse::<Value>().unwrap()
                    }
                });
                opcode::Op::new(opcode::OpcodeV1::try_from(opcode).unwrap(), operand)
            }
            _ => opcode::Op::new(opcode::OpcodeV1::try_from(opcode).unwrap(), None),
        }
    }
}
