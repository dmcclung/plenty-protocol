use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{self, Mint, MintTo, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// const SEED: &str = "state_v1";
const AUTHORITY_SEED: &str = "authority_v1";

#[program]
pub mod plenty {
    use super::*;

    pub fn init(ctx: Context<Init>, bump: u8, nonce: u8) -> ProgramResult {
        let mut state = ctx.accounts.state.load_init()?;
        state.nonce = nonce;
        state.bump = bump;
        state.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn create_loan(ctx: Context<CreateLoan>, amount: u64) -> ProgramResult {
        let mut loan = ctx.accounts.loan.load_init()?;
        loan.user = *ctx.accounts.user.key;
        loan.long_token_mint = *ctx.accounts.long_token_mint.to_account_info().key;
        loan.amount = amount;
        Ok(())
    }

    pub fn trade_long(ctx: Context<TradeLong>, size: u64) -> ProgramResult {
        // let loan = &mut ctx.accounts.loan.load_mut()?;
        let state = ctx.accounts.state.load_mut()?;

        let seeds = &[AUTHORITY_SEED.as_bytes(), &[state.nonce]];
        let signer = &[&seeds[..]];

        let cpi_ctx_mint: CpiContext<MintTo> = CpiContext::from(&*ctx.accounts).with_signer(signer);
        token::mint_to(cpi_ctx_mint, size.into())?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Init<'info> {
    #[account(init, seeds = [b"state_v1".as_ref()], bump = bump, payer = payer)]
    pub state: Loader<'info, State>,
    pub authority: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateLoan<'info> {
    #[account(init, payer = user)]
    pub loan: Loader<'info, Loan>,
    #[account(mut)]
    pub long_token_mint: Account<'info, Mint>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TradeLong<'info> {
    #[account(mut,
        seeds = [b"state_v1".as_ref()],
        bump = state.load()?.bump,
        constraint = state.to_account_info().owner == program_id
    )]
    pub state: Loader<'info, State>,
    #[account(constraint = authority.key == &state.load()?.authority)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub loan: Loader<'info, Loan>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&TradeLong<'info>> for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
    fn from(accounts: &TradeLong<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.mint.to_account_info(),
            to: accounts.user_token_account.to_account_info(),
            authority: accounts.authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[account(zero_copy)]
pub struct State {
    pub authority: Pubkey,
    pub nonce: u8,
    pub bump: u8,
}

impl Default for State {
    #[inline]
    fn default() -> State {
        State {
            authority: Pubkey::default(),
            nonce: 0,
            bump: 0,
        }
    }
}

#[account(zero_copy)]
pub struct Loan {
    pub user: Pubkey,
    pub long_token_mint: Pubkey,
    pub amount: u64,
}

impl Default for Loan {
    fn default() -> Loan {
        Loan {
            user: Pubkey::default(),
            long_token_mint: Pubkey::default(),
            amount: 0,
        }
    }
}
