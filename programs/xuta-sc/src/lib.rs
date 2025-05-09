use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;


use crate::instructions::*;
//use crate::state::*;
//pub use states::*;

declare_id!("9K9BEAMrDqauP8bDEHb19wuFZv5kCn8XjgeixWE7sf6K");

#[program]
pub mod xuta_sc {
    use super::*;

    pub fn initialize(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init(ctx.bumps.config)
    }
    pub fn start_campaign(ctx: Context<StartCampaign>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn buy_token(ctx: Context<BuyToken>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn claim_earnings(ctx: Context<ClaimEarnings>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn disable_institution(ctx: Context<DisableInstitution>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn finish_campaign(ctx: Context<FinishCampaign>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn init_earnings(ctx: Context<InitEarnings>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn init_institution(ctx: Context<InitInstitution>, name: String, contract: String) -> Result<()> {
        ctx.accounts.init_institution(name, contract)
    }
    
    pub fn init(ctx: Context<Init>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn pause_campaign(ctx: Context<PauseCampaign>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn set_authority(ctx: Context<SetAuthority>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn set_fee(ctx: Context<SetFee>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn set_institutions_authority(ctx: Context<SetInstitutionsAuthority>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        contract: String,
        image: String,
        ratio: u16,
        target_amount: u64,
        initial_date: i64,
        due_date: i64
    ) -> Result<()> {
        ctx.accounts.create_campaign(name, contract, image, ratio, target_amount, initial_date, due_date, ctx.bumps.campaign)
    }
    

}
