A bytecode structure, interpreter and optimizer for the Signature programming language.

# Structure

Bytecode consists of a **metadata section**, **procedures section**, and a **instruction section**.

-   Metadata section: Bytecode information and arguments are stored here.
-   Procedures section: A subprocedure is stored here. these procedures cannot be called from the parent procedure of the current procedure.
-   Instruction section: The instructions are stored here.

The first program started is also a procedure(the main procedure, `p0`), and it is the top level procedure of all procedures.

```js
// this is pseudo code, not real bytecode

.meta:
    name: "main"; // name of the procedure. when accessing the procedure, the name is not used, and the id is used instead.
    id: 0; // id of the procedure (p0)
    version: 1; // version of the Bytecode (default: 1)
.proc:
    .meta:
        name: "foo";
        id: 1;
    .proc:
        .meta:
            name: "bar";
            id: 2;
        .instr:
            // instructions
    .instr:
        // instructions
.instr:
    // instructions
```

# Opcodes

| Opcode        | Hex    | Parameters    | Description                                      |
| ------------- | ------ | ------------- | ------------------------------------------------ |
| `Noop`        | `0x00` |               | Does nothing.                                    |
| `Push`        | `0x01` | `...values`   | Push values onto the stack.                      |
| `Add`         | `0x02` |               | Pop two values, add them.                        |
| `Sub`         | `0x03` |               | Pop two values, subtract them.                   |
| `Mul`         | `0x04` |               | Pop two values, multiply them.                   |
| `Div`         | `0x05` |               | Pop two values, divide them.                     |
| `Mod`         | `0x06` |               | Pop two values, modulo them.                     |
| `Jump`        | `0x07` | `offset`      | Jump to offset (pointer).                        |
| `JumpIfFalse` | `0x08` | `offset`      | If top of stack is `0`, jump to offset.          |
| `Print`       | `0x09` | `length? = 1` | Pop items off the stack and print them.          |
| `PrintChar`   | `0x0A` | `length? = 1` | Pop items off the stack and print them as chars. |
