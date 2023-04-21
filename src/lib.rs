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

    fn parse_op(split_whitespace: Vec<&str>) -> opcode::Op {
        let opcode = split_whitespace[0];

        match opcode.to_lowercase().as_str() {
            "push" | "jump" | "jump_if_false" | "debug" => {
                let operand = split_whitespace.get(1).map(|s| {
                    if s.starts_with("0x") {
                        isize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
                    } else {
                        s.parse::<isize>().unwrap()
                    }
                });
                opcode::Op::new(opcode::OpcodeV1::from(opcode), operand)
            }
            _ => opcode::Op::new(opcode::OpcodeV1::from(opcode), None),
        }
    }
}
