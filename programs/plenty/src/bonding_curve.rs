use rust_decimal::prelude::*;

pub const RESERVE_RATIO: f64 = 0.10;

pub fn calculate_purchase_return(reserve_tokens_received: u64,
                                 reserve_token_balance: u64,
                                 continuous_token_supply: u64) -> Result<f64, &'static str> {
    // PurchaseReturn = ContinuousTokenSupply * ((1 + ReserveTokensReceived / ReserveTokenBalance) ^ (ReserveRatio) - 1)
    // https://yos.io/2018/11/10/bonding-curves

    let a = 1.0 + reserve_tokens_received as f64 / reserve_token_balance as f64;        
    let a_decimal = Decimal::from_f64(a).unwrap();
    let b = a_decimal.powf(RESERVE_RATIO) - Decimal::from_f64(1.0).unwrap();
    let purchase_return = Decimal::from_u64(continuous_token_supply).unwrap() * b;
    Ok(purchase_return.to_f64().unwrap())
}

pub fn calculate_sale_return(continuous_tokens_received: u64,
                             continuous_token_supply: u64,
                             reserve_token_balance: u64) -> Result<f64, &'static str> {
    // SaleReturn = ReserveTokenBalance * (1 - (1 - ContinuousTokensReceived / ContinuousTokenSupply) ^ (1 / (ReserveRatio)))
    // https://yos.io/2018/11/10/bonding-curves

    let a = 1.0 - continuous_tokens_received as f64 / continuous_token_supply as f64;
    let b = 1.0 / RESERVE_RATIO;
    
    let a_decimal = Decimal::from_f64(a).unwrap();
    let c_decimal = Decimal::from_f64(1.0).unwrap() - a_decimal.powf(b);

    let sale_return = Decimal::from_u64(reserve_token_balance).unwrap() * c_decimal;
    Ok(sale_return.to_f64().unwrap())
}

pub fn calculate_token_price(reserve_token_balance: u64,
                             continuous_token_supply: u64) -> Result<f64, &'static str> {
    // Continuous Token Price = Reserve Token Balance / (Continuous Token Supply x Reserve Ratio)
    // https://yos.io/2018/11/10/bonding-curves

    let token_price = reserve_token_balance as f64 / (continuous_token_supply  as f64 * RESERVE_RATIO);
    Ok(token_price)
}
