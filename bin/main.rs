use bytecode::{
    opcode::{Op, OpData, Opcode},
    vm::{Program, Vm},
};

fn main() {
    let opcodes = [
        Op::new(Opcode::Push, Some(OpData::Value(0x00))),
        Op::new(Opcode::JumpIfFalse, Some(OpData::Pointer(3))),
        Op::new(Opcode::Jump, Some(OpData::Pointer(6))),
        Op::new(Opcode::Push, Some(OpData::Value(0x46))),
        Op::new(Opcode::PrintChar, None),
        Op::new(Opcode::Jump, Some(OpData::Pointer(8))),
        Op::new(Opcode::Push, Some(OpData::Value(0x54))),
        Op::new(Opcode::PrintChar, None),
        Op::new(Opcode::Noop, None),
    ];
    let program = Program(&opcodes);

    println!("{program}");

    Vm::new(program).run().unwrap();
    println!();
}
