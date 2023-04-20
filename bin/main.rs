use bytecode::{
    opcode::{Op, OpcodeV1},
    rawbytes::RawBytes,
    vm::{Instructions, Program, Vm},
};

macro_rules! op {
    ($opcode:ident) => {
        Op::new(OpcodeV1::$opcode, None)
    };
    ($opcode:ident, $data:expr) => {
        Op::new(OpcodeV1::$opcode, Some($data))
    };
}

fn main() {
    #[rustfmt::skip]
    let opcodes = [
        op!(Push, /* 0x01 */ 0x01 /* 0x01 0x00 */),
        op!(JumpIfFalse, /* 0x08 */ 3  /* 0x02 3 */),
        op!(Jump /* 0x07 */, 9 /* 0x02 9 */),

        op!(Push, 0x65/* 0x01 0x65 */), // e
        op!(Push, 0x73/* 0x01 0x73 */), // s
        op!(Push, 0x6C/* 0x01 0x6C */), // l
        op!(Push, 0x61/* 0x01 0x61 */), // a
        op!(Push, 0x46/* 0x01 0x46 */), // F

        op!(Debug /* 0xFF */),

        op!(Push, 0x65/* 0x01 0x61 */), // e
        op!(Push, 0x75/* 0x01 0x75 */), // u
        op!(Push, 0x72/* 0x01 0x72 */), // r
        op!(Push, 0x54/* 0x01 0x54 */), // T

        op!(Debug /* 0xFF */),
        op!(Noop /* 0x00 */),
        // op!(Arguments /* 0x0C */),
        // op!(Debug /* 0xFF */),
    ];

    let rawbytes: RawBytes = Instructions(&opcodes).into();
    println!("{rawbytes:?}");

    let instructions: Instructions = rawbytes.try_into().unwrap();

    let program = Program(instructions);
    println!("{program}");

    Vm::new(program).run().unwrap();
}
