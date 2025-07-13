use anchor_lang::prelude::*;
use crate::state::{StakeConfig, UserAccount};
use anchor_spl::{
    token::{Token, Mint, TokenAccount},
    metadata::{Metadata, MetadataAccount, MasterEditionAccount}};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub mint_ata: Account<'info, TokenAccount>,
    pub mint_collection: Account<'info, Mint>,
    pub metadata_account: Account<'info, MetadataAccount>,
    pub master_edition_account: Account<'info, MasterEditionAccount>,
    #[account(mut)]
    pub stake_config: Account<'info, StakeConfig>,
    pub user_account: Account<'info, UserAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}