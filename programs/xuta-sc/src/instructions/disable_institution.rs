use anchor_lang::prelude::*;
use crate::{state::{Config, Institution}, error::CustomError};

pub fn disable_institution(ctx: Context<DisableInstitution>) -> Result<()> {
    // Check if there are any active campaigns
    require!(
        !ctx.accounts.institution.has_active_campaigns,
        CustomError::InstitutionHasActiveCampaigns
    );

    // Set the disabled flag to true
    ctx.accounts.institution.disabled = true;
    Ok(())
}

#[derive(Accounts)]
pub struct DisableInstitution<'info> {
    #[account(mut)]
    pub institution_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"institution".as_ref(), institution.name.as_ref()],
        bump = institution.bump,
    )]
    pub institution: Account<'info, Institution>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
        has_one = institution_authority @ CustomError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
}