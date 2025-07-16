use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub points_per_stake_per_day: u8,   // number of points per stake per day
    pub max_stake: u8,                  // maximum number of stakes a user can have
    pub freeze_period: u32,             // in days
    pub rewards_bump: u8,               // bump for the rewards mint
    pub bump: u8,                       // bump for the stake config
}