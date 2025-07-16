use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Max number of NFT staked")]
    MaxStakesReached,
    #[msg("Freeze period not met")]
    FreezePeriodNotMet,
}