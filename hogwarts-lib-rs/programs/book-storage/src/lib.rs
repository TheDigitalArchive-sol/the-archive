use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

declare_id!("Fk3qBVpk9KCYunBMcdaCcUm3wA9zj7rgBtdu7mBU6CHc");

const MAX_REALLOC_STEP: usize = 10_240;

#[program]
pub mod book_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_size: u32, total_chunks: u32) -> Result<()> {
        let storage_account = &mut ctx.accounts.storage_account;
        let total_storage_size = total_size * total_chunks; 
    
        storage_account.total_size = total_storage_size;
        storage_account.total_chunks = total_chunks;
        storage_account.data = Vec::new();
    
        msg!(
            "✅ Storage account initialized with total storage size {} bytes ({} chunks).",
            total_storage_size,
            total_chunks
        );
    
        Ok(())
    }
    
    pub fn store_data(ctx: Context<StoreData>, value: Vec<u8>) -> Result<()> {
        let storage_account = &mut ctx.accounts.storage_account;
        let payer = &ctx.accounts.user;
        let system_program = &ctx.accounts.system_program;

        let incoming_size: usize = value.len();
        let current_size = storage_account.total_size as usize;
        let new_size = current_size + incoming_size;

        let required_size = new_size + 8 + 4 + 4;
        let current_alloc_size = storage_account.to_account_info().data_len();
        let new_alloc_size = (current_alloc_size + MAX_REALLOC_STEP).max(required_size);

        let rent = Rent::get()?;
        let extra_rent_lamports = rent.minimum_balance(new_alloc_size) - storage_account.to_account_info().lamports();

        if extra_rent_lamports > 0 {
            let transfer_instruction = Transfer {
                from: payer.to_account_info(),
                to: storage_account.to_account_info(),
            };
            system_program::transfer(
                CpiContext::new(system_program.to_account_info(), transfer_instruction),
                extra_rent_lamports,
            )?;
        }

        storage_account.to_account_info().realloc(new_alloc_size, false)?;
        storage_account.data.extend_from_slice(&value);
        storage_account.total_size = new_size as u32;
        storage_account.total_chunks += 1;

        msg!("✅ Stored {} bytes (total: {})", incoming_size, storage_account.total_size);
        Ok(())
    }
}

#[account]
pub struct StorageAccount {
    pub total_size: u32,
    pub total_chunks: u32,
    pub data: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(total_size: u32, total_chunks: u32)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + 4 + 4 + (total_size as usize * total_chunks as usize)
    )]
    pub storage_account: Account<'info, StorageAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StoreData<'info> {
    #[account(mut)]
    pub storage_account: Account<'info, StorageAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Storage limit exceeded.")]
    StorageLimitExceeded,
}