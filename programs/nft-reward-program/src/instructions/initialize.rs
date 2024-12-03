use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use crate::state::RewardState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + RewardState::SIZE)]
    pub reward_state: Account<'info, RewardState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub monthly_pool: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, monthly_pool_bump: u8) -> Result<()> {
    let reward_state = &mut ctx.accounts.reward_state;
    
    reward_state.admin = ctx.accounts.admin.key();
    reward_state.monthly_pool = ctx.accounts.monthly_pool.key();
    reward_state.monthly_pool_bump = monthly_pool_bump;
    reward_state.is_frozen = false;
    reward_state.current_reward_amount = 0;
    reward_state.last_distribution_time = 0;
    reward_state.winner = Pubkey::default();
    reward_state.claim_deadline = 0;
    reward_state.is_claimed = false;
    reward_state.selected_distribution_option = 0;

    Ok(())
}
