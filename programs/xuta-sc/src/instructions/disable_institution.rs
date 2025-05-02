use anchor_lang::prelude::*;


pub fn disable_institution(_ctx: Context<DisableInstitution>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct DisableInstitution {}