use anchor_lang::prelude::*;

use crate::{state::{Config, Institution}, error::CustomError};

//Initializes a new institution account.

#[derive(Accounts)]
#[instruction(name: String, contract: String)]
pub struct InitInstitution<'info> {
    #[account(mut)]
    pub institution_authority: Signer<'info>,
    #[account(
        init,
        payer = institution_authority,
        space = 8 + Institution::INIT_SPACE,
        seeds = [b"institution".as_ref(), name.as_ref()],
        bump,
    )]
    pub institution: Account<'info, Institution>,
    #[account()]
    pub new_institution_authority: SystemAccount<'info>,
    #[account(
        seeds = [b"config".as_ref()], 
        bump,
        has_one = institution_authority @ CustomError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitInstitution<'info> {
    pub fn init_institution(&mut self, name: String, contract: String) -> Result<()> {
        self.institution.name = name;
        self.institution.authority = self.new_institution_authority.key();
        self.institution.contract = contract;
        self.institution.disabled = false;
        self.institution.has_active_campaigns = false;
        Ok(())
    }
}