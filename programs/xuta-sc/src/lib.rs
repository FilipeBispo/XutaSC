use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;


use crate::instructions::*;
//use crate::state::*;
//pub use states::*;

declare_id!("XUTAAsrE6AGc3xzvKtz6VNab6QuwVx41MD7HB7K5zVa");

#[program]
pub mod xuta_sc {
    use super::*;
    
    pub fn buy_token(ctx: Context<BuyToken>, amount: u64, receipt_bump: u8) -> Result<()> {
        BuyToken::buy_token(ctx, amount, receipt_bump)
    }
    
    pub fn claim_earnings(ctx: Context<ClaimEarnings>, amount: u64) -> Result<()> {
        ClaimEarnings::claim_earnings(ctx, amount)
    }
    
    pub fn disable_institution(ctx: Context<DisableInstitution>) -> Result<()> {
        instructions::disable_institution::disable_institution(ctx)
    }
    
    pub fn finish_campaign(ctx: Context<FinishCampaign>) -> Result<()> {
        instructions::finish_campaign::finish_campaign(ctx)
    }

    pub fn finish_earnings(ctx: Context<FinishEarnings>) -> Result<()> {
         instructions::finish_earnings::finish_earnings(ctx)
    }
    
    pub fn init_earnings(ctx: Context<InitEarnings>, ratio: u16) -> Result<()> {
        ctx.accounts.init_earnings(ratio)
    }

    pub fn init_institution(ctx: Context<InitInstitution>, name: String, contract: String, image: String, description: String) -> Result<()> {
        ctx.accounts.init_institution(name, contract, image, description)
    }
    
    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init(ctx.bumps.config)
    }
    
    pub fn pause_campaign(ctx: Context<PauseCampaign>) -> Result<()> {
        instructions::pause_campaign::pause_campaign(ctx)
    }
    
    pub fn set_authority(ctx: Context<SetAuthority>) -> Result<()> {
        instructions::set_authority::set_authority(ctx)
    }
    
    pub fn set_fee(ctx: Context<SetFee>, fee_pre: u16, fee_pos: u16) -> Result<()> {
        instructions::set_fee::set_fee(ctx, fee_pre, fee_pos)
    }
    
    pub fn set_institutions_authority(ctx: Context<SetInstitutionsAuthority>) -> Result<()> {
        instructions::set_institutions_authority::set_institutions_authority(ctx)
    }

    pub fn refund_receipt(ctx: Context<RefundReceipt>) -> Result<()> {
        RefundReceipt::refund_receipt(ctx)
    }

    pub fn redeem_token(ctx: Context<RedeemToken>) -> Result<()> {
        RedeemToken::redeem_token(ctx)
    }
    
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        contract: String,
        image: String,
        description: String,
        ratio: u16,
        target_amount: u64,
        initial_date: i64,
        due_date: i64
    ) -> Result<()> {
        ctx.accounts.create_campaign(name, contract, image, description, ratio, target_amount, initial_date, due_date, ctx.bumps.campaign)
    }
    

}
