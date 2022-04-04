use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Hotel with the same name already exists.")]
    Exists,
    #[error("All  rooms are taken.")]
    NoFreeRooms,
    #[error("This Hotel doesnot exist.")]
    NoHotel,
    #[error("Insufficient funds.")]
    InsufficientFunds,
    #[error("No funds.")]
    NoFunds,
}
