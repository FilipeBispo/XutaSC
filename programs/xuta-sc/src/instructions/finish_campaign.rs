use anchor_lang::prelude::*;
use crate::{state::{Campaign, CampaignStatus, Institution}, error::CustomError};

pub fn finish_campaign(ctx: Context<FinishCampaign>) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let institution = &mut ctx.accounts.institution;

    // Check if the campaign is active
    require!(
        campaign.status == CampaignStatus::Active,
        CustomError::CampaignNotActive
    );

    // Check if the campaign has reached its target
    if campaign.current_tokens >= campaign.target_amount {
        campaign.status = CampaignStatus::Successful;
    } else {
        campaign.status = CampaignStatus::Failed;
    }

    // Update institution's active campaigns status
    institution.has_active_campaigns = false;

    Ok(())
}

#[derive(Accounts)]
pub struct FinishCampaign<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        seeds = [b"campaign", campaign.mint_player.key().as_ref()],
        bump = campaign.campaign_bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        mut,
        seeds = [b"institution".as_ref(), institution.name.as_ref()],
        bump = institution.bump,
    )]
    pub institution: Account<'info, Institution>,
}