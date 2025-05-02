use anchor_lang::prelude::*;


pub fn buy_token(_ctx: Context<BuyToken>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct BuyToken {}