use anchor_lang::prelude::*;


pub fn submit_contract(_ctx: Context<SubmitContract>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitContract {}