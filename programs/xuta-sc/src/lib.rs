use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;


use crate::instructions::*;
//use crate::state::*;
//pub use states::*;

declare_id!("9K9BEAMrDqauP8bDEHb19wuFZv5kCn8XjgeixWE7sf6K");

#[program]
pub mod xuta_sc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
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
    
    pub fn init_institution(ctx: Context<InitInstitution>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
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
    
    pub fn init_contract(ctx: Context<InitContract>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    

}

#[derive(Accounts)]
pub struct Initialize {}
