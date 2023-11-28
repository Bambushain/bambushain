use crate::error::BambooError;

pub type PandaPartyErrorResult = Result<(), BambooError>;

pub type PandaPartyResult<T> = Result<T, BambooError>;
