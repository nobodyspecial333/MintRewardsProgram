use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::RewardState;

#[derive(Accounts)]
pub struct SetFrozen<'info> {
    #[account(mut)]
    pub reward_state: Account<'info, RewardState>,
    
    #[account(constraint = admin.key() == reward_state.admin @ ErrorCode::UnauthorizedAdmin)]
    pub admin: Signer<'info>,
}

pub fn set_frozen(ctx: Context<SetFrozen>, is_frozen: bool) -> Result<()> {
    let reward_state = &mut ctx.accounts.reward_state;
    reward_state.is_frozen = is_frozen;
    Ok(())
}
