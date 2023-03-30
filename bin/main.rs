use bytecode::{
    opcode::{Op, OpData, Opcode},
    vm::{Metadata, Procedure, Vm},
};

macro_rules! op {
    ($opcode:ident) => {
        Op::new(Opcode::$opcode, OpData::None)
    };
    ($opcode:ident, $data:expr) => {
        Op::new(Opcode::$opcode, $data)
    };
}

fn main() {
    let arguments = &[1, 2, 3];
    #[rustfmt::skip]
    let opcodes = [
        op!(Push, /* 0x01 */ OpData::Value(0x01) /* 0x01 0x00 */),
        op!(JumpIfFalse, /* 0x08 */ OpData::Pointer(3)  /* 0x02 3 */),
        op!(Jump /* 0x07 */, OpData::Pointer(9) /* 0x02 9 */),

        op!(Push, OpData::Value(0x65) /* 0x01 0x65 */), // e
        op!(Push, OpData::Value(0x73) /* 0x01 0x73 */), // s
        op!(Push, OpData::Value(0x6C) /* 0x01 0x6C */), // l
        op!(Push, OpData::Value(0x61) /* 0x01 0x61 */), // a
        op!(Push, OpData::Value(0x46) /* 0x01 0x46 */), // F

        op!(Debug /* 0xFF */),

        op!(Push, OpData::Value(0x65) /* 0x01 0x61 */), // e
        op!(Push, OpData::Value(0x75) /* 0x01 0x75 */), // u
        op!(Push, OpData::Value(0x72) /* 0x01 0x72 */), // r
        op!(Push, OpData::Value(0x54) /* 0x01 0x54 */), // T

        op!(Debug /* 0xFF */),
        op!(Noop /* 0x00 */),
        // op!(Arguments /* 0x0C */),
        // op!(Debug /* 0xFF */),
    ];

    let program = Procedure::new(Metadata::new("main", 0, arguments), &opcodes, &[]).unwrap();
    println!("{program}");

    Vm::new(program).run().unwrap();
    println!();
}
