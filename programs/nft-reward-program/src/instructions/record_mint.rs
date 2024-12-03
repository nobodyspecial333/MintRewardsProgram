use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::clock::Clock;
use crate::state::MintRecord;

#[derive(Accounts)]
pub struct RecordNFTMint<'info> {
    #[account(init, payer = minter, space = 8 + MintRecord::SIZE)]
    pub mint_record: Account<'info, MintRecord>,
    
    #[account(mut)]
    pub minter: Signer<'info>,
    
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub monthly_pool: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RecordNFTMint>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let mint_record = &mut ctx.accounts.mint_record;
    
    // Record mint details
    mint_record.minter = ctx.accounts.minter.key();
    mint_record.mint_time = clock.unix_timestamp;
    mint_record.amount = amount;

    // Calculate pool amounts (80% to monthly)
    let monthly_amount = amount * 80 / 100;
    
    // Transfer to monthly pool
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.monthly_pool.to_account_info(),
                authority: ctx.accounts.minter.to_account_info(),
            },
        ),
        monthly_amount,
    )?;

    Ok(())
}
