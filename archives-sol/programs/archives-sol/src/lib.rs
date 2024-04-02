use anchor_lang::prelude::*;

declare_id!("tPZbfwacaL9D9J6QyopjDoLtPYbGKtttvGAYwnF84AH");

#[program]
pub mod archives_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
