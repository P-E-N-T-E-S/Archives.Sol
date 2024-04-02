use anchor_lang::prelude::*;

declare_id!("7bYR2BPEbLE5uR22XsUito5GWeDMJAgnocgU4r88NNMA");

#[program]
pub mod archives_sol {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
