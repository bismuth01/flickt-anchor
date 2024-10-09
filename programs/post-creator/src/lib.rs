use anchor_lang::prelude::*;

declare_id!("6DZ3fUUrLKxFWjRzLRiM9XKnHG61qms8wPoyfhhnXD8C");

#[program]
pub mod post_creator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
