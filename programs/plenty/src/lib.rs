pub mod account;
pub mod context;

use crate::context::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};

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
        loan.short_token_mint = *ctx.accounts.short_token_mint.to_account_info().key;
        loan.amount = amount;
        Ok(())
    }

    pub fn trade_long(ctx: Context<TradeLong>, size: u64) -> ProgramResult {
        let state = ctx.accounts.state.load_mut()?;
        let seeds = &[AUTHORITY_SEED.as_bytes(), &[state.nonce]];
        let signer = &[&seeds[..]];

        let cpi_ctx_mint: CpiContext<MintTo> = CpiContext::from(&*ctx.accounts).with_signer(signer);
        token::mint_to(cpi_ctx_mint, size.into())?;
        Ok(())
    }
}
