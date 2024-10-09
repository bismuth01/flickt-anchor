use anchor_lang::prelude::*;

declare_id!("FRhDvDaKRNW8GL1uYjWgFJ4MCqSL4y3AfUyEksDv56LX");

#[program]
pub mod account_creator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
