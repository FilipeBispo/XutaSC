use anchor_lang::prelude::*;

use crate::state::Config;

impl<'info> Init<'info>{
    pub fn init(&mut self, config_bump: u8)-> Result<()>{
        let config = &mut self.config;
        config.authority = self.authority.key();
        config.institution_authority = self.authority.key();
        config.fee_pre = 0;
        config.fee_pos = 0;
        config.bump = config_bump;
          
        Ok(())
    }


}

#[derive(Accounts)]
//Initializes config account.
pub struct Init<'info> {
    //Payer
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [b"config".as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}