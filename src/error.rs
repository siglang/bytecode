use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[rustfmt::skip]
pub enum BytecodeErrorKind {
    #[error("Value is not provided.")] ValueNotProvided,
    #[error("Top of the stack is empty.")] StackIsEmpty,
    #[error("Invalid opcode: `{0}`.")] InvalidOpcode(u8),
    #[error("Invalid opcode: `{0}`.")] InvalidOpcode2(String),
    #[error("Call stack is empty.")] CallStackIsEmpty,
    #[error("Store index `{0}` is not found.")] StoreIndexNotFound(usize),
}

/// `(error, pointer)`
pub type BytecodeError = (BytecodeErrorKind, Option<usize>);
