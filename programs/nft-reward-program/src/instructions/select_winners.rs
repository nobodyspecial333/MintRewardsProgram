use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::clock::Clock;
use crate::error::ErrorCode;
use crate::state::RewardState;

#[derive(Accounts)]
pub struct SelectWinner<'info> {
    #[account(mut)]
    pub reward_state: Account<'info, RewardState>,
    
    #[account(mut)]
    pub monthly_pool: Account<'info, TokenAccount>,
    
    /// CHECK: Recent blockhash used for randomization
    pub recent_blockhash: AccountInfo<'info>,
    
    /// CHECK: Used to get mint records
    pub mint_records: AccountInfo<'info>,
    
    #[account(constraint = admin.key() == reward_state.admin @ ErrorCode::UnauthorizedAdmin)]
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<SelectWinner>) -> Result<()> {
    let reward_state = &mut ctx.accounts.reward_state;
    require!(!reward_state.is_frozen, ErrorCode::ProgramFrozen);

    let clock = Clock::get()?;
    
    // Check if it's the 15th of the month
    require!(
        is_fifteenth_of_month(clock.unix_timestamp),
        ErrorCode::InvalidSelectionTime
    );

    // Get eligible minters from last month
    let eligible_minters = get_eligible_minters(&ctx.accounts.mint_records)?;
    require!(!eligible_minters.is_empty(), ErrorCode::NoEligibleMinters);

    // Select random winner using recent block hash
    let recent_blockhash = ctx.accounts.recent_blockhash.key().to_bytes();
    let winner_index = (recent_blockhash[0] as usize) % eligible_minters.len();
    let winner = eligible_minters[winner_index];

    // Update reward state
    reward_state.winner = winner;
    reward_state.claim_deadline = clock.unix_timestamp + 30 * 24 * 60 * 60; // 30 days
    reward_state.is_claimed = false;
    reward_state.current_reward_amount = ctx.accounts.monthly_pool.amount;

    Ok(())
}

// Helper function to check if timestamp is on 15th of month
fn is_fifteenth_of_month(timestamp: i64) -> bool {
    let datetime = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    datetime.day() == 15
}

// Helper function to get eligible minters
fn get_eligible_minters(mint_records: &AccountInfo) -> Result<Vec<Pubkey>> {
    // Implementation needed: Parse mint records and return eligible minters
    // This is a placeholder that should be implemented based on your specific requirements
    Ok(vec![])
}
