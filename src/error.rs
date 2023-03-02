use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[rustfmt::skip]
pub enum BytecodeError {
    #[error("Value is not provided.")] ValueNotProvided,
    #[error("Pointer is not provided.")] PointerNotProvided,
    #[error("Top of the stack is empty.")] StackIsEmpty,
    #[error("Program is empty.")] ProgramIsEmpty,
    #[error("Invalid value.")] InvalidValue,
}

pub type BytecodeResult<T> = Result<T, BytecodeError>;
