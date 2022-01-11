use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_clock {
    use super::*;
    
    pub fn create_user(ctx: Context<CreateUser>, arguments: CreateUserArguments) -> ProgramResult {
        ctx.accounts.user.user_name = arguments.user_name;
        ctx.accounts.user.created_at = ctx.accounts.clock.unix_timestamp;
        ctx.accounts.user.updated_at = ctx.accounts.clock.unix_timestamp;
        ctx.accounts.user.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, arguments: UpdateUserArguments) -> ProgramResult {
        ctx.accounts.user.user_name = arguments.user_name;
        ctx.accounts.user.updated_at = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(arguments: CreateUserArguments)]
pub struct CreateUser<'info> {
    #[account(
        init,
        space = 100,
        payer = authority
    )]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(arguments: CreateUserArguments)]
pub struct UpdateUser<'info> {
    #[account(mut, has_one = authority)]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct User {
    pub authority: Pubkey,
    pub user_name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateUserArguments {
    pub user_name: String
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateUserArguments {
    pub user_name: String
}