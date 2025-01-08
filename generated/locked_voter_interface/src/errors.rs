use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum LockedVoterError {
    #[error("Lockup duration must at least be the min stake duration")]
    LockupDurationTooShort = 6000,
    #[error("Lockup duration must at most be the max stake duration")]
    LockupDurationTooLong = 6001,
    #[error("A voting escrow refresh cannot shorten the escrow time remaining")]
    RefreshCannotShorten = 6002,
    #[error("Escrow has not ended")]
    EscrowNotEnded = 6003,
    #[error("Maxlock is set")]
    MaxLockIsSet = 6004,
    #[error("Cannot set expiration less than the current time")]
    ExpirationIsLessThanCurrentTime = 6005,
    #[error("Locker is expired")]
    LockerIsExpired = 6006,
    #[error("Expiration is not zero")]
    ExpirationIsNotZero = 6007,
    #[error("Amount is zero")]
    AmountIsZero = 6008,
    #[error("Maxlock is not set")]
    MaxLockIsNotSet = 6009,
    #[error("Invalid amount for partial unstaking")]
    InvalidAmountForPartialUnstaking = 6010,
    #[error("Escrow has been ended")]
    EscrowHasBeenEnded = 6011,
    #[error("Invalid unstaking lock duration")]
    InvalidUnstakingLockDuration = 6012,
    #[error("Partial unstaking amount is not zero")]
    PartialUnstakingAmountIsNotZero = 6013,
    #[error("Partial unstaking has not ended")]
    PartialUnstakingIsNotEnded = 6014,
}
impl From<LockedVoterError> for ProgramError {
    fn from(e: LockedVoterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for LockedVoterError {
    fn type_of() -> &'static str {
        "LockedVoterError"
    }
}
impl PrintProgramError for LockedVoterError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
