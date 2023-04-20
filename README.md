# Opcodes
<!-- 
| Opcode          | Hex    | Parameters    | Description                                                   |
| --------------- | ------ | ------------- | ------------------------------------------------------------- |
| `Noop`          | `0x00` |               | Does nothing.                                                 |
| `Push`          | `0x01` | `...values`   | Push values onto the stack.                                   |
| `Add`           | `0x02` |               | Pop two values, add them.                                     |
| `Sub`           | `0x03` |               | Pop two values, subtract them.                                |
| `Mul`           | `0x04` |               | Pop two values, multiply them.                                |
| `Div`           | `0x05` |               | Pop two values, divide them.                                  |
| `Mod`           | `0x06` |               | Pop two values, modulo them.                                  |
| `Jump`          | `0x07` | `offset`      | Jump to offset (pointer).                                     |
| `JumpIfFalse`   | `0x08` | `offset`      | If top of stack is `0`, jump to offset.                       |
| ~~`Print`~~     | `0x09` | `length? = 1` | Pop items off the stack and print them. (deprecated)          |
| ~~`PrintChar`~~ | `0x0A` | `length? = 1` | Pop items off the stack and print them as chars. (deprecated) |
| `Call`          | `0x0B` | `id`          | Call procedure with id.                                       |
| `Arguments`     | `0x0C` |               | -->

TODO
