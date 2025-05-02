use anchor_lang::prelude::*;


pub fn set_authority(_ctx: Context<SetAuthority>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct SetAuthority {}