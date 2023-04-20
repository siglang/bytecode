use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[rustfmt::skip]
pub enum BytecodeError {
    #[error("Value is not provided.")] ValueNotProvided,
    #[error("Value kind is not provided.")] ValueKindNotProvided,
    #[error("Top of the stack is empty.")] StackIsEmpty,
    #[error("Invalid opcode: `{0}`.")] InvalidOpcode(u8),
}
