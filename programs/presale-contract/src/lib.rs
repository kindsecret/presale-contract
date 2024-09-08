use anchor_lang::prelude::*;

declare_id!("82Jdd3gmbvTNCbu2SYGRNkvcESCYgaA6NHBUzbLaH5oh");

mod errors;
mod instructions;

#[program]
pub mod presale_contract {
    use super::*;
    pub use super::instructions::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        instructions::initialize(ctx, token_name, token_symbol, token_uri)
    }
    
}

