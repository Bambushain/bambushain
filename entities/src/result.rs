use crate::error::SheefError;

pub type SheefErrorResult = Result<(), SheefError>;

pub type SheefResult<T> = Result<T, SheefError>;