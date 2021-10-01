use anchor_lang::prelude::*;

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
	pub short_token_mint: Pubkey,
	pub current_capital: u64, 
	pub required_capital: u64,
	pub long_token_circulation: u64, 
	pub reserve_long_token_balance: u64,
	pub short_token_circulation: u64, 
	pub reserve_short_token_balance: u64,
	pub long_token_price: u64, 
	pub short_token_price: u64,
	pub interest_rate: u64,
	pub amount: u64,
}

impl Default for Loan {
	fn default() -> Loan {
		Loan {
			user: Pubkey::default(),
			long_token_mint: Pubkey::default(),
			short_token_mint: Pubkey::default(),
			current_capital: 0,
			required_capital: 0,
			long_token_circulation: 0,
			reserve_long_token_balance: 0,
			short_token_circulation: 0,
			reserve_short_token_balance: 0,
			long_token_price: 0,
			short_token_price: 0,
			interest_rate: 0,
			amount: 0,
		}
	}
}
