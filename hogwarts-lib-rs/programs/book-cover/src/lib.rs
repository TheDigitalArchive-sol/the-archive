use anchor_lang::prelude::*;

declare_id!("BZSvLm3U9sZaMmG7rHuNv7SY3exrbRVE5LfpZcr5Tb99");

#[program]
pub mod book_cover {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
