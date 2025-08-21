use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("7dg8dMgyMHuY5zJ94AeRF8BbucZTDeioSJ76eWMRSPWd");

#[program]
pub mod staking_dapp {
    use super::*;

    pub fn initialize_stake_pool(ctx: Context<InitializeStakePool>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;
        stake_pool.staking_mint = ctx.accounts.staking_mint.key();
        stake_pool.staking_vault = ctx.accounts.staking_vault.key();
        stake_pool.reward_mint = ctx.accounts.reward_mint.key();
        stake_pool.reward_vault = ctx.accounts.reward_vault.key();
        stake_pool.admin = ctx.accounts.admin.key();
        stake_pool.bump = ctx.bumps.stake_pool; 
        Ok(())
    }

    pub fn create_user_stake_account(ctx: Context<CreateUserStakeAccount>) -> Result<()> {
        let user_stake_account = &mut ctx.accounts.user_stake_account;
        user_stake_account.user = ctx.accounts.user.key();
        user_stake_account.stake_pool = ctx.accounts.stake_pool.key();
        user_stake_account.staked_amount = 0;
        user_stake_account.last_staked_timestamp = 0;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let clock = Clock::get()?;

        let transfer_instruction = Transfer {
            from: ctx.accounts.user_staking_token_account.to_account_info(),
            to: ctx.accounts.staking_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, amount)?;

        ctx.accounts.user_stake_account.staked_amount += amount;
        ctx.accounts.user_stake_account.last_staked_timestamp = clock.unix_timestamp;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let stake_pool = &ctx.accounts.stake_pool;
        let user_stake_account = &mut ctx.accounts.user_stake_account;

        require!(user_stake_account.staked_amount >= amount, StakingError::InsufficientStake);

        let admin_key = stake_pool.admin;
        let seeds = &[
            b"stake_pool".as_ref(),
            admin_key.as_ref(),
            &[stake_pool.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_instruction = Transfer {
            from: ctx.accounts.staking_vault.to_account_info(),
            to: ctx.accounts.user_staking_token_account.to_account_info(),
            authority: stake_pool.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer
        );

        token::transfer(cpi_ctx, amount)?;

        user_stake_account.staked_amount -= amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStakePool<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32 + 32 + 32 + 32 + 1, 
        seeds = [b"stake_pool", admin.key().as_ref()],
        bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    pub staking_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        token::mint = staking_mint,
        token::authority = stake_pool,
    )]
    pub staking_vault: Account<'info, TokenAccount>,
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        token::mint = reward_mint,
        token::authority = stake_pool,
    )]
    pub reward_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateUserStakeAccount<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8 + 8)]
    pub user_stake_account: Account<'info, UserStakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub stake_pool: Account<'info, StakePool>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_staking_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_stake_account: Account<'info, UserStakeAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_staking_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_stake_account: Account<'info, UserStakeAccount>,
    #[account(
        seeds = [b"stake_pool", stake_pool.admin.as_ref()],
        bump = stake_pool.bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakePool {
    pub staking_mint: Pubkey,
    pub staking_vault: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
    pub admin: Pubkey,
    pub bump: u8, 
}

#[account]
pub struct UserStakeAccount {
    pub user: Pubkey,
    pub stake_pool: Pubkey,
    pub staked_amount: u64,
    pub last_staked_timestamp: i64,
}

#[error_code]
pub enum StakingError {
    #[msg("Insufficient staked amount to unstake.")]
    InsufficientStake,
}
