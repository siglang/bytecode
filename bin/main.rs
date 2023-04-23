use bytecode::{
    rawbytes::RawBytes,
    vm::{Instructions, Program, Vm},
    Code,
};

fn main() {
    let test = r#"
push 2 ;     0, stack: [ 2 ]
push 3 ;     1, stack: [ 2, 3 ]

proc 2 ;     2: square, jump to `pc (= 2)` + `instruction length (= 2)` + `1 (proc)` (= `pointer 5`)
    mul ;    3, stack: [ 6 ]
    return ; 4, call stack pop, jump to `pointer 6` (StackFrame)

call 3     ; 5, jump to `pointer 3`, call stack: [ StackFrame { pointer: 6 } ]
debug 0    ; 6, stack: [ 6 ]
        "#;

    let rawbytes: RawBytes = Instructions(Code(test).parse().0).into();
    println!("{rawbytes:?}");

    let instructions: Instructions = rawbytes.try_into().unwrap();

    let program = Program(instructions);

    Vm::new(program).run().unwrap();
}
