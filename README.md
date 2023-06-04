Bytecode instruction set and stack based virtual machine.

# Usage

```
Usage: bytecode <COMMAND>

Commands:
  b2r        ByteCode source code file -> RawBytes file  [ByteCode -> RawBytes]
  run        Interpret RawBytes file                     [RawBytes -> Execution]
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

`RawBytes` is data consisting of raw bytecode instructions and data.

```console
$ bytecode b2r -i examples/example1 -o examples/example1.raw
$ bytecode run -i examples/example1.raw
```

## Bytecode file

A small language exists to facilitate bytecode representation and is very simple to use.

```
opcode [operand] ; comment

push 2           ; Push 2 onto the stack.
push 0xFF        ; Push 255 onto the stack.
```

# Opcodes

| Opcode        | Hex             | Operand  | Stack                   | Description                                                                  |
| ------------- | --------------- | -------- | ----------------------- | ---------------------------------------------------------------------------- |
| `Noop`        | `0x00`          |          |                         | Does nothing.                                                                |
| `Push`        | `0x01`          | `value`  | `[.., value]`           | Push value onto the stack.                                                   |
| `Add`         | `0x02`          |          | `[.., A, B] -> [.., R]` | Pop two values, add them. (`A + B = R`)                                      |
| `Sub`         | `0x03`          |          | `[.., A, B] -> [.., R]` | Pop two values, subtract them. (`A + B = R`)                                 |
| `Mul`         | `0x04`          |          | `[.., A, B] -> [.., R]` | Pop two values, multiply them. (`A + B = R`)                                 |
| `Div`         | `0x05`          |          | `[.., A, B] -> [.., R]` | Pop two values, divide them. (`A + B = R`)                                   |
| `Mod`         | `0x06`          |          | `[.., A, B] -> [.., R]` | Pop two values, modulo them. (`A + B = R`)                                   |
| `Jump`        | `0x07`          | `offset` | `[.., ptr] -> [..]`     | Jump to offset (pointer).                                                    |
| `JumpIfFalse` | `0x08`          | `offset` | `[.., ptr] -> [..]`     | If top of stack is `0`, jump to offset.                                      |
| `GT`          | `0x09`          |          | `[.., A, B] -> [.., R]` | Pop two values, push `true`(`1`) if first is greater, else `false`(`0`).     |
| `LT`          | `0x0A`          |          | `[.., A, B] -> [.., R]` | Pop two values, push `true`(`1`) if first is less.                           |
| `GTE`         | `0x0B`          |          | `[.., A, B] -> [.., R]` | Pop two values, push `true`(`1`) if first is greater or equal.               |
| `LTE`         | `0x0C`          |          | `[.., A, B] -> [.., R]` | Pop two values, push `true`(`1`) if first is less or equal.                  |
| `EQ`          | `0x0D`          |          | `[.., A, B] -> [.., R]` | Pop two values, push `true`(`1`) if equal.                                   |
| `Proc`        | `0x0E`          | `length` | `[.., ptr] -> [..]`     | Instruction to delimit a procedure. instructions after `length` are omitted. |
| `Call`        | `0x0F`          | `offset` | `[.., ptr] -> [..]`     | Call procedure at `offset`.                                                  |
| `Ret`         | `0x10`          |          |                         | Return from procedure. See [procedures](#procedures) for more details.       |
| `Store`       | `0x11`          | `index`  | `[.., value] -> [..]`   | Pop value and store it at index.                                             |
| `Load`        | `0x12`          | `index`  | `[..] -> [.., value]`   | Load value at index and push it onto the stack.                              |
|               | `0x13` ~ `0xFD` |          |                         | Not used.                                                                    |
| `Exit`        | `0xFE`          |          |                         | Exit the program.                                                            |
| `Debug`       | `0xFF`          | `value`  | Description             | Print the stack. `-1` = pops all, `value` = pops `value` items.              |

## Procedures

`Proc` opcode takes a `length` operand, which is the number of instructions to skip after the `Proc` opcode.

Then, after executing the procedure in the CallStack using the `Call` instruction, the offset(pointer) to return to is stored.
After that, `Call` jump to the first instruction in the procedure and executes the procedure.

`Return` jumps to the return pointer stored in the CallStack.

```
proc 2     ; 0, The 2 instructions below are omitted.
    noop   ; 1
    return ; 2, Jump to the pointer (4) stored in the CallStack.

call 1     ; 3, After saving the pointer to return (4) to the CallStack, jump to the pointer 1 of the first instruction of the procedure.
noop       ; 4, Executed after the procedure runs.
```

### Example

See [examples](./examples) for more examples.
