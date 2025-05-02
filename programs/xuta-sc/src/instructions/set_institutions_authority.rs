use anchor_lang::prelude::*;


pub fn set_institutions_authority(_ctx: Context<SetInstitutionsAuthority>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct SetInstitutionsAuthority {}