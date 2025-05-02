use anchor_lang::prelude::*;


pub fn pause_campaign(_ctx: Context<PauseCampaign>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct PauseCampaign {}