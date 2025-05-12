use anchor_lang::prelude::*;
use crate::{state::{Campaign, CampaignStatus}, error::CustomError};

pub fn pause_campaign(ctx: Context<PauseCampaign>) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;

    // Check if the campaign is active
    require!(
        campaign.status == CampaignStatus::Active,
        CustomError::CampaignNotActive
    );

    // Check if the campaign has started
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    require!(
        now >= campaign.initial_date,
        CustomError::CampaignNotStarted
    );

    // Check if the campaign hasn't ended
    require!(
        now <= campaign.due_date,
        CustomError::CampaignEnded
    );

    // Update campaign status to Paused
    campaign.status = CampaignStatus::Paused;

    Ok(())
}

#[derive(Accounts)]
pub struct PauseCampaign<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"campaign", campaign.mint_player.key().as_ref()],
        bump = campaign.campaign_bump,
    )]
    pub campaign: Account<'info, Campaign>,
}