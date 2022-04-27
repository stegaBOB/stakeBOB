use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum StakeBobError {
    /// account doesnt match the expected PDA address
    #[error("Derived key invalid!")]
    DerivedKeyInvalid,

    /// data type mismatch on deserialization
    #[error("Data type mismatch")]
    DataTypeMismatch,

    /// NFT trying to be staked isn't in a collection
    #[error("NFT not in a collection")]
    NotInCollection,

    /// collection on NFT about to be staked is not verified
    #[error("Collection not verified")]
    CollectionNotVerified,
}

impl PrintProgramError for StakeBobError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<StakeBobError> for ProgramError {
    fn from(e: StakeBobError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for StakeBobError {
    fn type_of() -> &'static str {
        "Error Thingy"
    }
}
