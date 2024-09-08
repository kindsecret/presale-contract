use anchor_lang::prelude::*;

declare_id!("82Jdd3gmbvTNCbu2SYGRNkvcESCYgaA6NHBUzbLaH5oh");

#[program]
pub mod presale_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
