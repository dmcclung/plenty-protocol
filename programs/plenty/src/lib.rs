pub mod account;
pub mod context;
pub mod interest;
pub mod bonding_curve;

use crate::context::*;
use crate::interest::{ calculate_interest_rate, DECIMALS };
use crate::bonding_curve::{ 
    calculate_purchase_return, 
    calculate_sale_return, 
    calculate_token_price 
};
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

        // buy tokens
        let mut loan = ctx.accounts.loan.load_init()?;
        // figure out token price
        let token_price = calculate_token_price(loan.reserve_long_token_balance, loan.long_token_circulation);
        // figure out solana value transfered
        
        // figure out how many tokens to mint
        
        // mint the value

        let cpi_ctx_mint: CpiContext<MintTo> = CpiContext::from(&*ctx.accounts).with_signer(signer);
        token::mint_to(cpi_ctx_mint, size.into())?;

        // set token price paid
        // update circulation

        // update interest rate

        let interest_rate = calculate_interest_rate(loan.current_capital, 
                                                    loan.required_capital,
                                                    loan.long_token_circulation, 
                                                    loan.short_token_circulation, 
                                                    loan.long_token_price, 
                                                    loan.short_token_price).unwrap();
        loan.interest_rate = (interest_rate * DECIMALS) as u64;
        
        // TODO: How to calculate interest owed?
        // Interest rate is per block?

        Ok(())
    }

    pub fn trade_short(ctx: Context<TradeShort>, size: u64) -> ProgramResult {
        let state = ctx.accounts.state.load_mut()?;
        let seeds = &[AUTHORITY_SEED.as_bytes(), &[state.nonce]];
        let signer = &[&seeds[..]];

        let cpi_ctx_mint: CpiContext<MintTo> = CpiContext::from(&*ctx.accounts).with_signer(signer);
        token::mint_to(cpi_ctx_mint, size.into())?;
        Ok(())
    }
}
