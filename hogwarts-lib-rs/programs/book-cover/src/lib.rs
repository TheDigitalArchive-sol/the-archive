use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::{Instruction, AccountMeta};
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::types::{DataV2, Creator};
use mpl_token_metadata::instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs, CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs};

declare_id!("CLcPEsey6Ej423NZWnX7WnxVvE6Adirry8psgET3tiiG");

#[program]
pub mod book_cover {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted!");

        let creator = vec![
            Creator {
                address: solana_program::pubkey::Pubkey::new_from_array(creator_key.to_bytes()),
                verified: false,
                share: 100,
            },
            Creator {
                address: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.mint_authority.key().to_bytes()),
                verified: false,
                share: 0,
            },
        ];

        let symbol = "SYM".to_string();
        let metadata_accounts = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let metadata_instruction = Instruction {
            program_id: ctx.accounts.token_metadata_program.key(),
            accounts: metadata_accounts
                .iter()
                .map(|account| AccountMeta {
                    pubkey: account.key(),
                    is_signer: account.is_signer,
                    is_writable: account.is_writable,
                })
                .collect(),
            data: CreateMetadataAccountV3 {
                metadata: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.metadata.key().to_bytes()),
                mint: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.mint.key().to_bytes()),
                mint_authority: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.mint_authority.key().to_bytes()),
                payer: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.payer.key().to_bytes()),
                update_authority: (solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.payer.key().to_bytes()), true),
                system_program: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.system_program.key().to_bytes()),
                rent: Some(solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.rent.key().to_bytes())),
            }
            .instruction(CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: title,
                    symbol,
                    uri,
                    seller_fee_basis_points: 1,
                    creators: Some(creator),
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            })
            .data,
        };

        invoke(
            &metadata_instruction,  // Correct Anchor instruction usage
            metadata_accounts.as_slice(),
        )?;
        msg!("Metadata Account Created!");

        let master_edition_accounts = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let master_edition_instruction = Instruction {
            program_id: ctx.accounts.token_metadata_program.key(),
            accounts: master_edition_accounts
                .iter()
                .map(|account| AccountMeta {
                    pubkey: account.key(),
                    is_signer: account.is_signer,
                    is_writable: account.is_writable,
                })
                .collect(),
            data: CreateMasterEditionV3 {
                edition: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.master_edition.key().to_bytes()),
                mint: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.mint.key().to_bytes()),
                update_authority: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.payer.key().to_bytes()),
                mint_authority: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.mint_authority.key().to_bytes()),
                metadata: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.metadata.key().to_bytes()),
                payer: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.payer.key().to_bytes()),
                token_program: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.token_program.key().to_bytes()),
                system_program: solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.system_program.key().to_bytes()),
                rent: Some(solana_program::pubkey::Pubkey::new_from_array(ctx.accounts.rent.key().to_bytes())),
            }
            .instruction(CreateMasterEditionV3InstructionArgs { max_supply: None })
            .data,
        };

        invoke(
            &master_edition_instruction,  // Correct Anchor instruction usage
            master_edition_accounts.as_slice(),
        )?;
        
        msg!("Master Edition NFT Minted!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is an unchecked account because we will initialize it as a mint account.
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    
    /// CHECK: This is an unchecked account, verified within the instruction.
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This account is not validated because it will be the token account of the mint.
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: This account is UncheckedAccount.
    pub token_metadata_program: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: AccountInfo.
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: AccountInfo.
    pub rent: AccountInfo<'info>,

    /// CHECK: This is an unchecked account because it's used for the Master Edition.
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}
