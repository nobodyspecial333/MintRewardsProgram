use anchor_lang::prelude::*;

#[account]
pub struct RewardState {
    pub admin: Pubkey,
    pub monthly_pool: Pubkey,
    pub monthly_pool_bump: u8,
    pub is_frozen: bool,
    pub current_reward_amount: u64,
    pub last_distribution_time: i64,
    pub winner: Pubkey,
    pub claim_deadline: i64,
    pub is_claimed: bool,
    pub selected_distribution_option: u8,
}

#[account]
pub struct MintRecord {
    pub minter: Pubkey,
    pub mint_time: i64,
    pub amount: u64,
}

#[account]
pub struct Distribution {
    pub winner: Pubkey,
    pub total_amount: u64,
    pub remaining_amount: u64,
    pub total_claimed: u64,
    pub total_burned: u64,
    pub total_returned: u64,
    pub last_claim_time: i64,
    pub option: u8,
    pub schedule: DistributionSchedule,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct DistributionSchedule {
    pub payment_amount: u64,
    pub interval_seconds: i64,
    pub payments_remaining: u8,
    pub next_payment_time: i64,
}

impl RewardState {
    pub const SIZE: usize = 32 + 32 + 1 + 1 + 8 + 8 + 32 + 8 + 1 + 1;
}

impl MintRecord {
    pub const SIZE: usize = 32 + 8 + 8;
}

impl Distribution {
    pub const SIZE: usize = 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + std::mem::size_of::<DistributionSchedule>();
}
