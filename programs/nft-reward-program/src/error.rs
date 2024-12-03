use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Program is currently frozen")]
    ProgramFrozen,
    #[msg("Selection can only occur on the 15th of the month")]
    InvalidSelectionTime,
    #[msg("No eligible minters found")]
    NoEligibleMinters,
    #[msg("Invalid winner")]
    InvalidWinner,
    #[msg("Reward already claimed")]
    AlreadyClaimed,
    #[msg("Claim period has expired")]
    ClaimExpired,
    #[msg("Invalid distribution option")]
    InvalidDistributionOption,
    #[msg("Unauthorized admin action")]
    UnauthorizedAdmin,
}
