use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;
//pub use states::*;

declare_id!("9K9BEAMrDqauP8bDEHb19wuFZv5kCn8XjgeixWE7sf6K");

#[program]
pub mod xuta_sc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
