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
