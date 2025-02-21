use anchor_lang::prelude::*;

declare_id!("J3NbsLGamAXywfUm8ecKJedAXa7FoNAhSoq3XcNBCRAD");

#[program]
pub mod book_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
