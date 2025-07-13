#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
mod state;
mod instructions;
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

}
