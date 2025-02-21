pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FnopRi7du13hHM4b6K9kUeZwguqgznHUSaYGpPjg3sqJ");

#[program]
pub mod hogwarts_lib_rs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
