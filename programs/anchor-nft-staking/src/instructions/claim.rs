use anchor_lang::prelude::*;
use crate::state::{UserAccount, StakeConfig};
use anchor_spl::{
    token::{Token, Mint, TokenAccount, mint_to, MintTo},
    associated_token::AssociatedToken,
};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"rewards".as_ref(), stake_config.key().as_ref()],
        bump = stake_config.rewards_bump,
    )]
    pub rewards_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_mint_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = stake_config.bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {

        // mint rewards to user
        let amount_to_mint: u64 = self.user_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32);

        let seeds = &[
            b"config".as_ref(),
            &[self.stake_config.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_mint_ata.to_account_info(),
            authority: self.stake_config.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, amount_to_mint)?;

        // update user account
        self.user_account.points = 0;

        Ok(())
    }
}