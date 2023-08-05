use crate::error::PandaPartyError;

pub type PandaPartyErrorResult = Result<(), PandaPartyError>;

pub type PandaPartyResult<T> = Result<T, PandaPartyError>;