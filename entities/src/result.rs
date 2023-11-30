use crate::error::BambooError;

pub type BambooErrorResult = Result<(), BambooError>;

pub type BambooResult<T> = Result<T, BambooError>;
