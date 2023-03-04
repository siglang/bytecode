A bytecode structure, interpreter and optimizer for the Signature programming language.

# Opcodes

| Opcode        | Parameters    | Description                                      |
| ------------- | ------------- | ------------------------------------------------ |
| `Noop`        |               | Does nothing.                                    |
| `Push`        | `...values`   | Push values onto the stack.                      |
| `Add`         |               | Pop two values, add them.                        |
| `Sub`         |               | ^, subtract.                                     |
| `Mul`         |               | ^, multiply.                                     |
| `Div`         |               | ^, divide.                                       |
| `Mod`         |               | ^, modulo.                                       |
| `Jump`        | `offset`      | Jump to offset (pointer).                        |
| `JumpIfFalse` | `offset`      | If top of stack is `0`, ^                        |
| `Print`       | `length? = 1` | Pop items off the stack and print them.          |
| `PrintChar`   | `length? = 1` | Pop items off the stack and print them as chars. |
