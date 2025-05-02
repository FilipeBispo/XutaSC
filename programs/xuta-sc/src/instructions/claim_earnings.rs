use anchor_lang::prelude::*;


pub fn claim_earnings(_ctx: Context<ClaimEarnings>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimEarnings {}