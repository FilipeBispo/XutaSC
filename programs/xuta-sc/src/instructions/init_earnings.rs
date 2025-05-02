use anchor_lang::prelude::*;


pub fn init_earnings(_ctx: Context<InitEarnings>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct InitEarnings {}