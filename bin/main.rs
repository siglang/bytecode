use bytecode::{
    opcode::{Op, OpData, Opcode},
    vm::{Program, Vm},
};

macro_rules! op {
    ($opcode:ident) => {
        Op::new(Opcode::$opcode, None)
    };
    ($opcode:ident, $($data:expr),*) => {
        Op::new(Opcode::$opcode, Some(&[$($data),*]))
    };
}

fn main() {
    let opcodes = [
        op!(Push, OpData::Value(0x00)),
        op!(JumpIfFalse, OpData::Pointer(3)),
        op!(Jump, OpData::Pointer(6)),
        op!(
            Push,
            OpData::Value(0x65),
            OpData::Value(0x73),
            OpData::Value(0x6C),
            OpData::Value(0x61),
            OpData::Value(0x46)
        ), // False
        op!(PrintChar, OpData::Length(5)),
        op!(Jump, OpData::Pointer(8)),
        op!(
            Push,
            OpData::Value(0x65),
            OpData::Value(0x75),
            OpData::Value(0x72),
            OpData::Value(0x54)
        ), // True
        op!(PrintChar, OpData::Length(4)),
        op!(Noop),
    ];
    let program = Program(&opcodes);

    println!("{program}");

    Vm::new(program).run().unwrap();
    println!();
}
