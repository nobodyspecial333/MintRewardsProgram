pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod nft_reward_program {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, monthly_pool_bump: u8) -> Result<()> {
        instructions::initialize::handler(ctx, monthly_pool_bump)
    }

    pub fn record_nft_mint(ctx: Context<RecordNFTMint>, amount: u64) -> Result<()> {
        instructions::record_mint::handler(ctx, amount)
    }

    pub fn select_winner(ctx: Context<SelectWinner>) -> Result<()> {
        instructions::select_winner::handler(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, distribution_option: u8) -> Result<()> {
        instructions::claim_reward::handler(ctx, distribution_option)
    }

    pub fn set_frozen(ctx: Context<SetFrozen>, is_frozen: bool) -> Result<()> {
        instructions::admin::set_frozen(ctx, is_frozen)
    }
}
