use anchor_lang::prelude::*;
use crate::state::StakeConfig;
use anchor_spl::token::{Mint, Token};


#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [b"config".as_ref()],
        bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"rewards".as_ref(), stake_config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = stake_config,
    )]
    pub rewards_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitConfig<'info> {
    pub fn init_config(&mut self, points_per_stake_per_day: u8, max_stake: u8, freeze_period: u32, bumps: &InitConfigBumps) -> Result<()> {
        self.stake_config.set_inner(StakeConfig {
            points_per_stake_per_day,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.stake_config,
        });
        Ok(())
    }
}