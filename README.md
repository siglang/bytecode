A bytecode structure, interpreter and optimizer for the Signature programming language.

# Opcodes

| Opcode        | Parameters | Description                                                                                        |
| ------------- | ---------- | -------------------------------------------------------------------------------------------------- |
| `Noop`        |            | Does nothing.                                                                                      |
| `Push`        | `value`    | Pushes a value onto the stack.                                                                     |
| `Add`         |            | Pops two values from the stack, adds them and pushes the result.                                   |
| `Sub`         |            | Pops two values from the stack, subtracts them and pushes the result.                              |
| `Mul`         |            | Pops two values from the stack, multiplies them and pushes the result.                             |
| `Div`         |            | Pops two values from the stack, divides them and pushes the result.                                |
| `Mod`         |            | Pops two values from the stack, calculates the modulus and pushes the result.                      |
| `Jump`        | `offset`   | Jumps to the specified offset (pointer).                                                           |
| `JumpIfFalse` | `offset`   | Pops a value from the stack. If the value is false (`0`), jumps to the specified offset (pointer). |
| `Print`       |            | Pops a value from the stack and prints it.                                                         |
| `PrintChar`   |            | Pops a value from the stack and prints it as a character.                                          |
