use anchor_lang::prelude::*;


pub fn finish_campaign(_ctx: Context<FinishCampaign>) -> Result<()> {
    // implementation to be made
    Ok(())
}

#[derive(Accounts)]
pub struct FinishCampaign {}