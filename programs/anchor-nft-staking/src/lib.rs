#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
mod state;
mod instructions;
mod errors;
use instructions::*;

declare_id!("Hi6y7Y83NdWWVdtUhHiSjZTNmXRVfoaUcUDU7c7uAS6E");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn InitUser(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn InitConfig(ctx: Context<InitConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn Stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn Unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn ClaimRewards(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
    
    

}
