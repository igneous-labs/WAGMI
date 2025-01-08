use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum GovernError {
    #[error("Invalid vote side.")]
    InvalidVoteSide = 6000,
    #[error("Invalid proposal type.")]
    InvalidProposalType = 6001,
    #[error("The owner of the smart wallet doesn't match with current.")]
    GovernorNotFound = 6002,
    #[error(
        "The proposal cannot be activated since it has not yet passed the voting delay."
    )]
    VotingDelayNotMet = 6003,
    #[error("Only drafts can be canceled.")]
    ProposalNotDraft = 6004,
    #[error("The proposal must be active.")]
    ProposalNotActive = 6005,
    #[error("Max option is invalid")]
    InvalidMaxOption = 6006,
    #[error("Proposal is not YesNo.")]
    NotYesNoProposal = 6007,
    #[error("Proposal is not Option.")]
    NotOptionProposal = 6008,
    #[error("Invalid option descriptions.")]
    InvalidOptionDescriptions = 6009,
}
impl From<GovernError> for ProgramError {
    fn from(e: GovernError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for GovernError {
    fn type_of() -> &'static str {
        "GovernError"
    }
}
impl PrintProgramError for GovernError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
