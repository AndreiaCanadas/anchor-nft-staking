use anchor_lang::prelude::*;
use crate::{
    state::{StakeConfig, UserAccount, StakeAccount},
    errors::StakeError
};
use anchor_spl::{
    token::{
        revoke,
        Revoke,
        Token, 
        Mint, 
        TokenAccount},
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        }, 
        Metadata,
        MasterEditionAccount
    }
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"metadata".as_ref(), b"edition".as_ref(), metadata_program.key().as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = metadata_program.key(),
    )]
    pub master_edition_account: Account<'info, MasterEditionAccount>,
    #[account(
        mut,
        close = user,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), stake_config.key().as_ref()],
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,
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
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}
impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {

        // check if the freeze period has elapsed
        let time_elapsed_days: u32 = ((Clock::get()?.unix_timestamp - self.stake_account.start_time) / (60 * 60 * 24)) as u32;
        require!(time_elapsed_days >= self.stake_config.freeze_period, StakeError::FreezePeriodNotMet);

        // unfreeze the NFT
        let metadata_program = &self.metadata_program.to_account_info();
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.master_edition_account.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();

        let seeds = &[
            b"stake".as_ref(),
            self.mint.to_account_info().key.as_ref(),
            self.stake_config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        ThawDelegatedAccountCpi::new(
            metadata_program, 
            ThawDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            }
        ).invoke_signed(signer_seeds)?;

        // revoke the NFT
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            source: self.mint_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        revoke(cpi_ctx)?;

        // update user account
        self.user_account.points += self.stake_config.points_per_stake_per_day as u32 * time_elapsed_days;
        self.user_account.amount_staked -= 1;

        Ok(())
    }
}