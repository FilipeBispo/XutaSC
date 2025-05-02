use anchor_lang::prelude::*;


pub fn init_institution(_ctx: Context<InitInstitution>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct InitInstitution {}