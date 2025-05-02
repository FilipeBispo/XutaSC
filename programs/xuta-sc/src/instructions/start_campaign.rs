use anchor_lang::prelude::*;
use crate::state::{Campaign, CampaignStatus};


#[derive(Accounts)]
#[instruction(name: String)]
pub struct StartCampaign<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        seeds = [b"campaign", name.as_bytes()],
        bump,
        space = Campaign::INIT_SPACE,
    )]
    pub campaign: Account<'info, Campaign>,
    pub system_program: Program<'info, System>,
}


impl<'info> StartCampaign<'info>{
    pub fn start_campaign(&mut self)-> Result<()>{
        
        Ok(())
    }


}