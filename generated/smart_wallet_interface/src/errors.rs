use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum SmartWalletError {
    #[error("The given owner is not part of this smart wallet.")]
    InvalidOwner = 6000,
    #[error("Estimated execution block must satisfy delay.")]
    InvalidEta = 6001,
    #[error("Delay greater than the maximum.")]
    DelayTooHigh = 6002,
    #[error("Not enough owners signed this transaction.")]
    NotEnoughSigners = 6003,
    #[error("Transaction is past the grace period.")]
    TransactionIsStale = 6004,
    #[error("Transaction hasn't surpassed time lock.")]
    TransactionNotReady = 6005,
    #[error("The given transaction has already been executed.")]
    AlreadyExecuted = 6006,
    #[error("Threshold must be less than or equal to the number of owners.")]
    InvalidThreshold = 6007,
    #[error("Owner set has changed since the creation of the transaction.")]
    OwnerSetChanged = 6008,
    #[error("Subaccount does not belong to smart wallet.")]
    SubaccountOwnerMismatch = 6009,
    #[error("Number of signers is not zero.")]
    NumSignerIsNotZero = 6010,
}
impl From<SmartWalletError> for ProgramError {
    fn from(e: SmartWalletError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SmartWalletError {
    fn type_of() -> &'static str {
        "SmartWalletError"
    }
}
impl PrintProgramError for SmartWalletError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
