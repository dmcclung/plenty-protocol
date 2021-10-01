
pub const VARIABLE_RATE_SLOPE1: f64 = -0.9322;
pub const VARIABLE_RATE_SLOPE2: f64 = -0.1000;
pub const VARIABLE_RATE_SLOPE3: f64 = -0.0090;
pub const BASE_VARIABLE_RATE: f64 = 00.0;

pub const OPTIMAL_FUNDING_RATE1: f64 = 0.60;
pub const OPTIMAL_FUNDING_RATE2: f64 = 1.00;

pub const DECIMALS: f64 = u64::pow(10, 9) as f64;

pub fn calculate_interest_rate(current_capital: u64,
                               required_capital: u64,
                               long_token_circulation: u64,
                               short_token_circulation: u64,
                               long_token_price: u64,
                               short_token_price: u64) -> Result<f64, &'static str> {
    let long_market_cap: f64 = (long_token_circulation * long_token_price) as f64;
    let short_market_cap: f64 = (short_token_circulation * short_token_price) as f64;
    let long_percentage: f64 = long_market_cap / (long_market_cap + short_market_cap);

    let capital_ratio: f64 = current_capital as f64 / required_capital as f64;

    let funding_rate: f64 = capital_ratio * 100.0 * long_percentage;

    let interest_rate;
    if funding_rate < OPTIMAL_FUNDING_RATE1 {
        interest_rate = VARIABLE_RATE_SLOPE1 * funding_rate + BASE_VARIABLE_RATE;
    } else if funding_rate < OPTIMAL_FUNDING_RATE2 {
        interest_rate = VARIABLE_RATE_SLOPE2 * funding_rate + BASE_VARIABLE_RATE;
    } else {
        interest_rate = VARIABLE_RATE_SLOPE3 * funding_rate + BASE_VARIABLE_RATE;
    }

    Ok(interest_rate.abs())
}


