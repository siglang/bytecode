use bytecode::{
    opcode::{Op, OpcodeV1},
    vm::Instructions,
    Pointer, Value,
};

#[derive(Debug, Clone)]
pub struct Code<'a>(pub &'a str);

impl<'a> Code<'a> {
    pub fn parse(&self) -> Instructions<'a> {
        let mut instructions = Vec::new();

        for line in self.0.trim().lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let split_whitespace = line.split_whitespace().collect::<Vec<_>>();

            instructions.push(Code::parse_op(split_whitespace));
        }

        Instructions(Box::leak(instructions.into_boxed_slice()))
    }

    fn parse_op(op: Vec<&str>) -> Op {
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
                Op::new(OpcodeV1::try_from(opcode).unwrap(), operand)
            }
            "push" => {
                let operand = op.get(1).map(|s| {
                    if s.starts_with("0x") {
                        Value::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
                    } else {
                        s.parse::<Value>().unwrap()
                    }
                });
                Op::new(OpcodeV1::try_from(opcode).unwrap(), operand)
            }
            _ => Op::new(OpcodeV1::try_from(opcode).unwrap(), None),
        }
    }
}
