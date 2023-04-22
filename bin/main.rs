use bytecode::{
    rawbytes::RawBytes,
    vm::{Instructions, Program, Vm},
    Code,
};

fn main() {
    let test = r#"
push 2 ;          0
push 3 ;          1
lte ;             2
jump_if_false 5 ; 3
jump 12 ;         4

push 0x65 ;       5
push 0x73 ;       6
push 0x6C ;       7
push 0x61 ;       8
push 0x46 ;       9
debug 0x00 ;      10

jump 15 ;         11

push 0x65 ;       12
push 0x75 ;       13
push 0x72 ;       14
push 0x54 ;       15
debug 0x00 ;      16

noop ;            17
        "#;

    let rawbytes: RawBytes = Instructions(Code(test).parse().0).into();
    println!("{rawbytes:?}");

    let instructions: Instructions = rawbytes.try_into().unwrap();

    let program = Program(instructions);

    Vm::new(program).run().unwrap();
}
