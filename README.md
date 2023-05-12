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
|               | `0x11` ~ `0xFD` |          |                         | Not used.                                                                    |
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

```
push 2     ; 0, stack: [ 2 ]
push 3     ; 1, stack: [ 2, 3 ]

proc 2     ; 2: square, jump to `pc (= 2)` + `instruction length (= 2)` + `1 (proc)` (= `pointer 5`)
    mul    ; 3, stack: [ 6 ]
    return ; 4, call stack pop, jump to `pointer 6` (StackFrame)

call 3     ; 5, jump to `pointer 3`, call stack: [ StackFrame { pointer: 6 } ]
debug 0    ; 6, stack: [ 6 ]
```
