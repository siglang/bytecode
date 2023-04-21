use bytecode::{
    rawbytes::RawBytes,
    vm::{Instructions, Program, Vm},
    Code,
};

fn main() {
    let test = r#"
push 0x01
jump_if_false 3
jump 9

push 0x65
push 0x73
push 0x6C
push 0x61
push 0x46

debug 0x00

push 0x65
push 0x75
push 0x72
push 0x54

debug 0x00
noop
        "#;

    let rawbytes: RawBytes = Instructions(Code(test).parse().0).into();
    println!("{rawbytes:?}");

    let instructions: Instructions = rawbytes.try_into().unwrap();

    let program = Program(instructions);

    Vm::new(program).run().unwrap();
}
