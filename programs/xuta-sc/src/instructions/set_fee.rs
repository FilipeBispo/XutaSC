use anchor_lang::prelude::*;


pub fn set_fee(_ctx: Context<SetFee>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct SetFee {}