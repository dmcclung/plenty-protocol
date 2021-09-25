use crate::account::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{Mint, MintTo, TokenAccount};

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
	#[account(init, mint::decimals = 0, mint::authority = authority, payer = user)]
	pub long_token_mint: Account<'info, Mint>,
	#[account(init, mint::decimals = 0, mint::authority = authority, payer = user)]
	pub short_token_mint: Account<'info, Mint>,
	#[account(signer)]
	pub user: AccountInfo<'info>,
	pub authority: AccountInfo<'info>,
	pub system_program: AccountInfo<'info>,
	pub token_program: AccountInfo<'info>,
	pub rent: Sysvar<'info, Rent>,
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
	#[account(mut, constraint = mint.to_account_info().key == &loan.load()?.long_token_mint)]
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

#[derive(Accounts)]
pub struct TradeShort<'info> {
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
	#[account(mut, constraint = mint.to_account_info().key == &loan.load()?.short_token_mint)]
	pub mint: Account<'info, Mint>,
	pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&TradeShort<'info>> for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
	fn from(accounts: &TradeShort<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
		let cpi_accounts = MintTo {
			mint: accounts.mint.to_account_info(),
			to: accounts.user_token_account.to_account_info(),
			authority: accounts.authority.to_account_info(),
		};
		let cpi_program = accounts.token_program.to_account_info();
		CpiContext::new(cpi_program, cpi_accounts)
	}
}
