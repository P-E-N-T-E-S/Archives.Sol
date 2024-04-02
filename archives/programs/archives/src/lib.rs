use anchor_lang::prelude::*;

declare_id!("4VXdnDCoG585gSRvR7S3ZtjZQjGinnyt8waBrCEWUGcm");

#[program]
pub mod archives {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
