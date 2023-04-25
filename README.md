# Opcodes

| Opcode        | Hex             | Parameters | Description                                                                  |
| ------------- | --------------- | ---------- | ---------------------------------------------------------------------------- |
| `Noop`        | `0x00`          |            | Does nothing.                                                                |
| `Push`        | `0x01`          | `value`    | Push value onto the stack.                                                   |
| `Add`         | `0x02`          |            | Pop two values, add them.                                                    |
| `Sub`         | `0x03`          |            | Pop two values, subtract them.                                               |
| `Mul`         | `0x04`          |            | Pop two values, multiply them.                                               |
| `Div`         | `0x05`          |            | Pop two values, divide them.                                                 |
| `Mod`         | `0x06`          |            | Pop two values, modulo them.                                                 |
| `Jump`        | `0x07`          | `offset`   | Jump to offset (pointer).                                                    |
| `JumpIfFalse` | `0x08`          | `offset`   | If top of stack is `0`, jump to offset.                                      |
| `GT`          | `0x09`          |            | Pop two values, push `true`(`1`) if first is greater, else `false`(`0`).     |
| `LT`          | `0x0A`          |            | Pop two values, push `true`(`1`) if first is less.                           |
| `GTE`         | `0x0B`          |            | Pop two values, push `true`(`1`) if first is greater or equal.               |
| `LTE`         | `0x0C`          |            | Pop two values, push `true`(`1`) if first is less or equal.                  |
| `EQ`          | `0x0D`          |            | Pop two values, push `true`(`1`) if equal.                                   |
| `Proc`        | `0x0E`          | `length`   | Instruction to delimit a procedure. instructions after `length` are omitted. |
| `Call`        | `0x0F`          | `offset`   | Call procedure at `offset`.                                                  |
| `Ret`         | `0x10`          |            | Return from procedure. See [procedures](#procedures) for more details.       |
|               | `0x11` ~ `0xFD` |            | Reserved for future use.                                                     |
| `Exit`        | `0xFE`          |            | Exit the program.                                                            |
| `Debug`       | `0xFF`          | `value`    | Print the stack. `-1` = pops all, `value` = pops `value` items.              |

## Procedures

<!-- TODO -->
