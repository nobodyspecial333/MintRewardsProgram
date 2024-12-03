use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Burn};
use crate::error::ErrorCode;
use crate::state::{RewardState, Distribution, DistributionSchedule};
use solana_program::clock::Clock;

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub reward_state: Account<'info, RewardState>,
    
    #[account(mut)]
    pub monthly_pool: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Token mint for burning
    #[account(mut)]
    pub token_mint: AccountInfo<'info>,
    
    pub winner: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    
    /// Distribution state account to track scheduled payments
    #[account(
        init_if_needed,
        payer = winner,
        space = 8 + Distribution::SIZE,
        seeds = [b"distribution", reward_state.key().as_ref(), winner.key().as_ref()],
        bump
    )]
    pub distribution: Account<'info, Distribution>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimReward>, distribution_option: u8) -> Result<()> {
    let reward_state = &mut ctx.accounts.reward_state;
    let clock = Clock::get()?;
    
    // Validate basic claiming conditions
    require!(!reward_state.is_frozen, ErrorCode::ProgramFrozen);
    require!(ctx.accounts.winner.key() == reward_state.winner, ErrorCode::InvalidWinner);
    require!(!reward_state.is_claimed, ErrorCode::AlreadyClaimed);
    require!(clock.unix_timestamp <= reward_state.claim_deadline, ErrorCode::ClaimExpired);

    // Initialize distribution tracking
    let distribution = &mut ctx.accounts.distribution;
    distribution.winner = ctx.accounts.winner.key();
    distribution.total_amount = reward_state.current_reward_amount;
    distribution.remaining_amount = reward_state.current_reward_amount;
    distribution.last_claim_time = clock.unix_timestamp;
    distribution.option = distribution_option;

    // Process distribution based on option
    match distribution_option {
        1 => process_immediate_claim(ctx, distribution)?,
        2 => process_monthly_distribution(ctx, distribution)?,
        3 => process_quarterly_distribution(ctx, distribution)?,
        4 => process_yearly_distribution(ctx, distribution)?,
        _ => return Err(ErrorCode::InvalidDistributionOption.into()),
    }

    reward_state.is_claimed = true;
    reward_state.selected_distribution_option = distribution_option;
    
    Ok(())
}

fn process_immediate_claim(
    ctx: Context<ClaimReward>,
    distribution: &mut Account<'_, Distribution>,
) -> Result<()> {
    let total_amount = distribution.total_amount;
    
    // Calculate amounts
    let claim_amount = total_amount * 50 / 100; // 50%
    let burn_amount = total_amount * 25 / 100;  // 25%
    let return_amount = total_amount * 25 / 100; // 25%

    // Transfer immediate payment
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.monthly_pool.to_account_info(),
                to: ctx.accounts.winner_token_account.to_account_info(),
                authority: ctx.accounts.monthly_pool.to_account_info(),
            },
        ),
        claim_amount,
    )?;

    // Burn tokens
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.monthly_pool.to_account_info(),
                authority: ctx.accounts.monthly_pool.to_account_info(),
            },
        ),
        burn_amount,
    )?;

    // Update distribution state
    distribution.schedule = DistributionSchedule {
        payment_amount: claim_amount,
        interval_seconds: 0, // Immediate payment
        payments_remaining: 0,
        next_payment_time: 0,
    };
    
    distribution.remaining_amount = 0;
    distribution.total_claimed = claim_amount;
    distribution.total_burned = burn_amount;
    distribution.total_returned = return_amount;

    Ok(())
}

fn process_monthly_distribution(
    ctx: Context<ClaimReward>,
    distribution: &mut Account<'_, Distribution>,
) -> Result<()> {
    let total_amount = distribution.total_amount;
    let clock = Clock::get()?;
    
    // Calculate amounts
    let distribute_amount = total_amount * 70 / 100; // 70%
    let burn_amount = total_amount * 15 / 100;      // 15%
    let return_amount = total_amount * 15 / 100;    // 15%

    // Calculate monthly payment
    let payment_amount = distribute_amount / 6; // Split over 6 months
    
    // Burn tokens
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.monthly_pool.to_account_info(),
                authority: ctx.accounts.monthly_pool.to_account_info(),
            },
        ),
        burn_amount,
    )?;

    // Set up monthly schedule
    distribution.schedule = DistributionSchedule {
        payment_amount,
        interval_seconds: 30 * 24 * 60 * 60, // 30 days
        payments_remaining: 6,
        next_payment_time: clock.unix_timestamp + (30 * 24 * 60 * 60),
    };

    distribution.remaining_amount = distribute_amount;
    distribution.total_burned = burn_amount;
    distribution.total_returned = return_amount;

    Ok(())
}

fn process_quarterly_distribution(
    ctx: Context<ClaimReward>,
    distribution: &mut Account<'_, Distribution>,
) -> Result<()> {
    let total_amount = distribution.total_amount;
    let clock = Clock::get()?;
    
    // Calculate amounts
    let distribute_amount = total_amount * 85 / 100; // 85%
    let burn_amount = total_amount * 7 / 100;       // 7.5%
    let return_amount = total_amount * 7 / 100;     // 7.5%

    // Calculate quarterly payment
    let payment_amount = distribute_amount / 4; // Split over 4 quarters
    
    // Burn tokens
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.monthly_pool.to_account_info(),
                authority: ctx.accounts.monthly_pool.to_account_info(),
            },
        ),
        burn_amount,
    )?;

    // Set up quarterly schedule
    distribution.schedule = DistributionSchedule {
        payment_amount,
        interval_seconds: 90 * 24 * 60 * 60, // 90 days
        payments_remaining: 4,
        next_payment_time: clock.unix_timestamp + (90 * 24 * 60 * 60),
    };

    distribution.remaining_amount = distribute_amount;
    distribution.total_burned = burn_amount;
    distribution.total_returned = return_amount;

    Ok(())
}

fn process_yearly_distribution(
    ctx: Context<ClaimReward>,
    distribution: &mut Account<'_, Distribution>,
) -> Result<()> {
    let total_amount = distribution.total_amount;
    let clock = Clock::get()?;
    
    // Calculate bi-annual payment
    let payment_amount = total_amount / 2; // Split into 2 payments
    
    // Set up bi-annual schedule
    distribution.schedule = DistributionSchedule {
        payment_amount,
        interval_seconds: 180 * 24 * 60 * 60, // 180 days
        payments_remaining: 2,
        next_payment_time: clock.unix_timestamp + (180 * 24 * 60 * 60),
    };

    distribution.remaining_amount = total_amount;
    distribution.total_burned = 0;
    distribution.total_returned = 0;

    Ok(())
}

// New instruction to claim scheduled payments
pub fn claim_scheduled_payment(ctx: Context<ClaimScheduledPayment>) -> Result<()> {
    let distribution = &mut ctx.accounts.distribution;
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp >= distribution.schedule.next_payment_time,
        ErrorCode::PaymentNotDue
    );
    require!(
        distribution.schedule.payments_remaining > 0,
        ErrorCode::NoRemainingPayments
    );

    let payment_amount = distribution.schedule.payment_amount;

    // Transfer scheduled payment
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.monthly_pool.to_account_info(),
                to: ctx.accounts.winner_token_account.to_account_info(),
                authority: ctx.accounts.monthly_pool.to_account_info(),
            },
        ),
        payment_amount,
    )?;

    // Update distribution state
    distribution.schedule.payments_remaining -= 1;
    distribution.schedule.next_payment_time += distribution.schedule.interval_seconds;
    distribution.remaining_amount = distribution.remaining_amount.saturating_sub(payment_amount);
    distribution.total_claimed += payment_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimScheduledPayment<'info> {
    #[account(mut)]
    pub distribution: Account<'info, Distribution>,
    
    #[account(mut)]
    pub monthly_pool: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    
    pub winner: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}
