use anchor_client::solana_sdk::{ program_pack::Pack, signature::Keypair, signer::Signer};

// use anchor_lang::prelude::{AccountInfo, Pubkey};
// use anchor_spl::associated_token::{get_associated_token_address, spl_associated_token_account};
// use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3};
// use mpl_token_metadata::types::Creator;
use std::rc::Rc;
// use std::str::FromStr;
use std::sync::Arc;
use anchor_spl::{associated_token::spl_associated_token_account, metadata::{mpl_token_metadata::types::{Creator, DataV2}, CreateMasterEditionV3, CreateMetadataAccountsV3}, token::{initialize_mint2, spl_token, Mint}};
// use mpl_token_metadata::instructions::{CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs};

// pub async fn create_mint_account(
//     client: &Client<Rc<&Arc<Keypair>>>,
//     payer: &Keypair,
//     mint: &Keypair,
//     mint_authority_pubkey: &anchor_lang::prelude::Pubkey,
//     token_program_id: anchor_lang::prelude::Pubkey,
// ) -> anchor_lang::prelude::Pubkey {
//     let mint_pubkey: anchor_lang::prelude::Pubkey = mint.pubkey();
//     println!("🪙 Mint Account Public Key: {:?}", mint_pubkey);
//     println!("🪙 Mint Authority Public Key: {:?}", mint_authority_pubkey);
//     println!("🪙 Mint Token Program ID: {:?}", token_program_id);

//     let program = client.program(token_program_id).unwrap();

//     // Get required rent exemption
//     let rent_lamports = program
//         .rpc()
//         .get_minimum_balance_for_rent_exemption(Mint::LEN)
//         .unwrap();

//     println!("💰 Required Rent Exemption: {}", rent_lamports);

//     // Step 1: Create Mint Account
//     let create_mint_account_ix = anchor_client::solana_sdk::system_instruction::create_account(
//         &payer.pubkey(),
//         &mint_pubkey,
//         rent_lamports,
//         Mint::LEN as u64,
//         &token_program_id,
//     );

//     // Step 2: Initialize Mint Account
//     let init_mint_ix = spl_token::instruction::initialize_mint(
//         &token_program_id,
//         &mint_pubkey,
//         mint_authority_pubkey,
//         None, // No freeze authority
//         0, // 0 decimal places for NFT
//     )
//     .unwrap();

//     // Step 3: Create Associated Token Account (ATA)
//     let token_account = get_associated_token_address(&payer.pubkey(), &mint_pubkey);
//     println!("✅ Token Account Exists: {}", token_account);

//     let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
//         &payer.pubkey(),
//         &payer.pubkey(),
//         &mint_pubkey,
//         &token_program_id,
//     );

//     // Step 4: Execute Transaction
//     let tx = anchor_client::solana_sdk::transaction::Transaction::new_signed_with_payer(
//         &[create_mint_account_ix, init_mint_ix, create_ata_ix], // Include ATA
//         Some(&payer.pubkey()),
//         &[payer, mint],
//         program.rpc().get_latest_blockhash().unwrap(),
//     );

//     let signature = program.rpc().send_and_confirm_transaction(&tx).unwrap();

//     println!("✅ Mint Account Ready at: {}", mint_pubkey);
//     println!("✅ Mint Account Created! Signature: {:?}", signature);

//     mint_pubkey
// }


use anchor_lang::{prelude::{AccountInfo, CpiContext, Pubkey}, system_program};
use anchor_client::Client;
use anchor_spl::{associated_token::get_associated_token_address, metadata::{create_master_edition_v3, create_metadata_accounts_v3}};

pub fn mint_nft(
    client: &Client<Rc<&Arc<Keypair>>>,
    payer_keypair: &Keypair,
    mint_keypair: &Keypair,
    program_id: Pubkey,
    uri: String,
    name: String,
    symbol: String,
    metaplex_program_id: Pubkey,
    token_program_id: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let program = client.program(program_id).unwrap();

    println!("🪙 Minting NFT with PDA-based mint authority");

    // PDA-based mint authority
    let (mint_pda, bump) = Pubkey::find_program_address(&[b"mint"], &program_id);
    println!("🪙 PDA Mint Authority: {:?}", mint_pda);

    // Compute PDAs for Metadata & Master Edition
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[b"metadata", &metaplex_program_id.to_bytes(), &mint_keypair.pubkey().to_bytes()],
        &metaplex_program_id,
    );

    let (master_edition_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            &metaplex_program_id.to_bytes(),
            &mint_keypair.pubkey().to_bytes(),
            b"edition",
        ],
        &metaplex_program_id,
    );

    println!("🪙 Metadata PDA Pubkey: {:?}", metadata_pda);
    println!("🪙 Master Edition PDA Pubkey: {:?}", master_edition_pda);

    let associated_token_address = get_associated_token_address(&payer_keypair.pubkey(), &mint_keypair.pubkey());

    // Create Mint Account
    let rent_lamports = program.rpc().get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;
    let create_mint_ix = spl_token::solana_program::system_instruction::create_account(
        &payer_keypair.pubkey(),
        &mint_keypair.pubkey(),
        rent_lamports,
        spl_token::state::Mint::LEN as u64,
        &token_program_id,
    );

    // Initialize Mint with PDA authority
    let initialize_mint_ix: spl_token::solana_program::instruction::Instruction = spl_token::instruction::initialize_mint2(
        &token_program_id,
        &mint_keypair.pubkey(),
        &payer_keypair.pubkey(),
        None,
        0, // 0 decimals = NFT
    )?;
    // Create Associated Token Account
    // Create Associated Token Account
    let create_token_account_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer_keypair.pubkey(),
        &payer_keypair.pubkey(),
        &mint_keypair.pubkey(),
        &token_program_id,
    );

    // Mint NFT to Associated Token Account
    let mint_nft_ix = spl_token::instruction::mint_to(
        &token_program_id,
        &mint_keypair.pubkey(),
        &associated_token_address,
        &payer_keypair.pubkey(),
        &[],
        1, // Mint 1 token
    )?;

    // Metadata Data Structure
    let metadata_data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0, // No royalties
        creators: Some(vec![Creator {
            address: payer_keypair.pubkey(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    // Create Metadata Account
    let tmp_mkp = mint_keypair.pubkey().clone();
    let tmp_pkp = payer_keypair.pubkey().clone();
    // Define independent mutable values for each AccountInfo instance
    let mut lamports_metadata = 0;
    let mut data_metadata: Vec<u8> = vec![];

    let mut lamports_mint = 0;
    let mut data_mint: Vec<u8> = vec![];

    let mut lamports_authority0 = 0;
    let mut data_authority0: Vec<u8> = vec![];

    let mut lamports_authority1 = 0;
    let mut data_authority1: Vec<u8> = vec![];

    let mut lamports_payer = 0;
    let mut data_payer: Vec<u8> = vec![];

    let mut lamports_system = 0;
    let mut data_system: Vec<u8> = vec![];

    let mut lamports_rent = 0;
    let mut data_rent: Vec<u8> = vec![];

    let metadata_accounts = CreateMetadataAccountsV3 {
        metadata: AccountInfo::new(&metadata_pda, false, true, &mut lamports_metadata, &mut data_metadata, &metaplex_program_id, false, 0),
        mint: AccountInfo::new(&tmp_mkp, false, true, &mut lamports_mint, &mut data_mint, &token_program_id, false, 0),
        mint_authority: AccountInfo::new(&mint_pda, true, false, &mut lamports_authority0, &mut data_authority0, &program_id, false, 0),
        payer: AccountInfo::new(&tmp_pkp, true, true, &mut lamports_payer, &mut data_payer, &system_program::ID, false, 0),
        update_authority: AccountInfo::new(&mint_pda, true, false, &mut lamports_authority1, &mut data_authority1, &program_id, false, 0),
        system_program: AccountInfo::new(&system_program::ID, false, false, &mut lamports_system, &mut data_system, &system_program::ID, false, 0),
        rent: AccountInfo::new(&system_program::ID, false, false, &mut lamports_rent, &mut data_rent, &system_program::ID, false, 0),
    };

    let mut lamports_metaplex = 0;
    let mut data_metaplex: Vec<u8> = vec![];
    
    let metaplex_program_info = AccountInfo::new(
        &metaplex_program_id,
        false,
        false,
        &mut lamports_metaplex,
        &mut data_metaplex,
        &metaplex_program_id,
        false,
        0
    );
    // Define persistent signer seeds
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[bump]]];

    // Construct the CPI Context
    let cpi_ctx_metadata = CpiContext::new(
        metaplex_program_info.clone(), // ✅ Reuse instead of creating new
        metadata_accounts
    ).with_signer(signer_seeds); // PDA as signer

    // Invoke Metadata Account Creation
    create_metadata_accounts_v3(
        cpi_ctx_metadata,
        metadata_data, 
        true,  // Is mutable
        true,  // Update authority is signer
        None,  // No collection details
    )?;


    // Define independent mutable values for each AccountInfo instance
    let mut lamports_edition = 0;
    let mut data_edition: Vec<u8> = vec![];

    let mut lamports_metadata = 0;
    let mut data_metadata: Vec<u8> = vec![];

    let mut lamports_mint = 0;
    let mut data_mint: Vec<u8> = vec![];

    let mut lamports_mint_auth = 0;
    let mut data_mint_auth: Vec<u8> = vec![];

    let mut lamports_payer = 0;
    let mut data_payer: Vec<u8> = vec![];

    let mut lamports_update_auth = 0;
    let mut data_update_auth: Vec<u8> = vec![];

    let mut lamports_system = 0;
    let mut data_system: Vec<u8> = vec![];

    let mut lamports_token = 0;
    let mut data_token: Vec<u8> = vec![];

    let mut lamports_rent = 0;
    let mut data_rent: Vec<u8> = vec![];
    
    // Create Master Edition Account
    let master_edition_accounts = CreateMasterEditionV3 {
        edition: AccountInfo::new(&master_edition_pda, false, true, &mut lamports_edition, &mut data_edition, &metaplex_program_id, false, 0),
        metadata: AccountInfo::new(&metadata_pda, false, true, &mut lamports_metadata, &mut data_metadata, &metaplex_program_id, false, 0),
        mint: AccountInfo::new(&tmp_mkp, false, true, &mut lamports_mint, &mut data_mint, &token_program_id, false, 0),
        mint_authority: AccountInfo::new(&mint_pda, true, false, &mut lamports_mint_auth, &mut data_mint_auth, &program_id, false, 0),
        payer: AccountInfo::new(&tmp_pkp, true, true, &mut lamports_payer, &mut data_payer, &system_program::ID, false, 0),
        update_authority: AccountInfo::new(&mint_pda, true, false, &mut lamports_update_auth, &mut data_update_auth, &program_id, false, 0),
        system_program: AccountInfo::new(&system_program::ID, false, false, &mut lamports_system, &mut data_system, &system_program::ID, false, 0),
        token_program: AccountInfo::new(&spl_token::ID, false, false, &mut lamports_token, &mut data_token, &spl_token::ID, false, 0),
        rent: AccountInfo::new(&system_program::ID, false, false, &mut lamports_rent, &mut data_rent, &system_program::ID, false, 0),
    };

    
    let mut lamports_metaplex = 0;
    let mut data_metaplex: Vec<u8> = vec![];
    let metaplex_program_info = AccountInfo::new(
        &metaplex_program_id,
        false,
        false,
        &mut lamports_metaplex,
        &mut data_metaplex,
        &metaplex_program_id,
        false,
        0
    );

    let cpi_ctx_master_edition = CpiContext::new(
        metaplex_program_info, // ✅ Reuse instead of creating new
        master_edition_accounts
    ).with_signer(signer_seeds); // PDA as signer
    
    // Invoke Master Edition Account Creation
    create_master_edition_v3(
        cpi_ctx_master_edition,
        Some(0),  // Max supply (None for unlimited)
    )?;
    let instructions = vec![
        create_mint_ix,
        initialize_mint_ix,
        create_token_account_ix,
        mint_nft_ix,
    ];
    let recent_blockhash = program.rpc().get_latest_blockhash()?;
    let transaction = anchor_client::solana_sdk::transaction::Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer_keypair.pubkey()), // Payer of the transaction fees
        &[payer_keypair, mint_keypair], // Signers
        recent_blockhash,
    );
    let signature = program.rpc().send_and_confirm_transaction(&transaction)?;
    println!("✅ NFT Minted Successfully! Transaction Signature: {}", signature);

    Ok(())
}



// pub fn mint_nft(
//     client: &Client<Rc<&Arc<Keypair>>>,
//     payer_keypair: &Keypair,
//     mint_keypair: &Keypair,
//     program_id: anchor_lang::prelude::Pubkey,
//     uri: String,
//     name: String,
//     symbol: String,
//     metaplex_program_id: solana_program::pubkey::Pubkey,
//     token_program_id: anchor_lang::prelude::Pubkey,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let program = client.program(program_id).unwrap();
//     println!("🪙 Book Cover Program ID: {:?}", program_id);
//     println!("🪙 Metaplex Program ID: {:?}", metaplex_program_id);
//     println!("🪙 Mint Account Public Key: {:?}", mint_keypair.pubkey());
//     println!("🪙 Mint Authority Public Key: {:?}", payer_keypair.pubkey());
//     println!("🪙 Mint Token Program ID: {:?}", token_program_id);

//     // Compute PDAs for Metadata & Master Edition
//     // let metadata_pda: solana_sdk::pubkey::Pubkey = solana_program::pubkey::Pubkey::find_program_address(
//     //     &[b"metadata", &metaplex_program_id.to_bytes(), &mint_keypair.pubkey().to_bytes()],
//     //     &metaplex_program_id,
//     // )
//     // .0;

//     let (metadata_pda, _) = solana_program::pubkey::Pubkey::find_program_address(
//         &[b"metadata", &metaplex_program_id.to_bytes(), &mint_keypair.pubkey().to_bytes()],
//         &metaplex_program_id,
//     );
    
//     println!("🪙 Metadata PDA Pubkey: {:?}", metadata_pda);

//     let master_edition_pda: solana_sdk::pubkey::Pubkey = solana_program::pubkey::Pubkey::find_program_address(
//         &[
//             b"metadata",
//             &metaplex_program_id.to_bytes(),
//             &mint_keypair.pubkey().to_bytes(),
//             b"edition",
//         ],
//         &metaplex_program_id,
//     )
//     .0;

//     let associated_token_address = get_associated_token_address(&payer_keypair.pubkey(), &mint_keypair.pubkey());

//     // Create Mint Account
//     let create_mint_ix: spl_token::solana_program::instruction::Instruction = anchor_client::solana_sdk::system_instruction::create_account(
//         &payer_keypair.pubkey(),
//         &mint_keypair.pubkey(),
//         program.rpc().get_minimum_balance_for_rent_exemption(82)?,
//         82,
//         &token_program_id,
//     );

//     // Initialize Mint
//     let initialize_mint_ix: spl_token::solana_program::instruction::Instruction = spl_token::instruction::initialize_mint2(
//         &token_program_id,
//         &mint_keypair.pubkey(),
//         &payer_keypair.pubkey(),
//         None,
//         0, // 0 decimals = NFT
//     )?;

//     // Create Associated Token Account
//     let create_token_account_ix = spl_associated_token_account::instruction::create_associated_token_account(
//         &payer_keypair.pubkey(),
//         &payer_keypair.pubkey(),
//         &mint_keypair.pubkey(),
//         &token_program_id,
//     );

//     // Mint NFT to Associated Token Account
//     let mint_nft_ix = spl_token::instruction::mint_to(
//         &token_program_id,
//         &mint_keypair.pubkey(),
//         &associated_token_address,
//         &payer_keypair.pubkey(),
//         &[],
//         1, // Mint 1 token
//     )?;

//     // Create Metadata Account
// // Create Metadata Account (Fixed Order)
// let create_metadata_ix: spl_token::solana_program::instruction::Instruction = spl_token::solana_program::instruction::Instruction {
//     program_id: anchor_lang::prelude::Pubkey::from_str(&metaplex_program_id.to_string()).unwrap(),
//     accounts: vec![
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&metadata_pda.to_string()).unwrap(), false), // ✅ Metadata PDA (must be new)
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&mint_keypair.pubkey().to_string()).unwrap(), false), // NFT Mint
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&payer_keypair.pubkey().to_string()).unwrap(), true), // ✅ Mint authority (must sign)
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&payer_keypair.pubkey().to_string()).unwrap(), true), // ✅ Payer (must sign)
//         anchor_lang::prelude::AccountMeta::new_readonly(anchor_lang::prelude::Pubkey::from_str(&metaplex_program_id.to_string()).unwrap(), false), // ✅ Metaplex Token Metadata Program
//         anchor_lang::prelude::AccountMeta::new_readonly(anchor_lang::prelude::Pubkey::from_str(&solana_program::system_program::ID.to_string()).unwrap(), false), // ✅ System Program (Needed for creation)
//         anchor_lang::prelude::AccountMeta::new_readonly(anchor_lang::prelude::Pubkey::from_str(&solana_program::sysvar::rent::ID.to_string()).unwrap(), false), // ✅ Rent Sysvar
//     ],
//     data: mpl_token_metadata::instructions::CreateMetadataAccountV3 {
//         metadata: metadata_pda,
//         mint: solana_program::pubkey::Pubkey::from_str_const(&mint_keypair.pubkey().to_string()),
//         mint_authority: solana_program::pubkey::Pubkey::from_str_const(&payer_keypair.pubkey().to_string()),
//         payer: solana_program::pubkey::Pubkey::from_str_const(&payer_keypair.pubkey().to_string()),
//         update_authority: (solana_program::pubkey::Pubkey::from_str_const(&payer_keypair.pubkey().to_string()), true),
//         system_program: solana_program::system_program::ID,
//         rent: Some(solana_program::sysvar::rent::ID),
//     }
//     .instruction(mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
//         data: mpl_token_metadata::types::DataV2 {
//             name,
//             symbol,
//             uri,
//             seller_fee_basis_points: 0,
//             creators: Some(vec![mpl_token_metadata::types::Creator {
//                 address: solana_program::pubkey::Pubkey::from_str(&payer_keypair.pubkey().to_string()).unwrap(),
//                 verified: true,
//                 share: 100,
//             }]),
//             collection: None,
//             uses: None,
//         },
//         is_mutable: true,
//         collection_details: None,
//     })
//     .data,
// };


// // Create Master Edition Account (Fixed Order)
// let create_master_edition_ix: spl_token::solana_program::instruction::Instruction = spl_token::solana_program::instruction::Instruction {
//     program_id: anchor_lang::prelude::Pubkey::from(metaplex_program_id.to_bytes()),

//     accounts: vec![
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&master_edition_pda.to_string()).unwrap(), false),  // Edition PDA
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&mint_keypair.pubkey().to_string()).unwrap(), false), // NFT Mint
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&metadata_pda.to_string()).unwrap(), false),  // ✅ Metadata PDA (Fixed position)
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&payer_keypair.pubkey().to_string()).unwrap(), true), // ✅ Payer should be signer
//         anchor_lang::prelude::AccountMeta::new(anchor_lang::prelude::Pubkey::from_str(&payer_keypair.pubkey().to_string()).unwrap(), true), // ✅ Mint authority should be signer
//         anchor_lang::prelude::AccountMeta::new_readonly(anchor_lang::prelude::Pubkey::from_str(&spl_token::ID.to_string()).unwrap(), false), // SPL Token Program
//         anchor_lang::prelude::AccountMeta::new_readonly(anchor_lang::prelude::Pubkey::from_str(&solana_program::system_program::ID.to_string()).unwrap(), false), // System Program
//         anchor_lang::prelude::AccountMeta::new_readonly(anchor_lang::prelude::Pubkey::from_str(&solana_program::sysvar::rent::ID.to_string()).unwrap(), false), // Rent Sysvar
//     ],
//     data: CreateMasterEditionV3 {
//         edition: master_edition_pda,
//         mint: solana_program::pubkey::Pubkey::from_str_const(&mint_keypair.pubkey().to_string()),
//         update_authority: solana_program::pubkey::Pubkey::from_str_const(&payer_keypair.pubkey().to_string()), // ✅ Fix update authority
//         mint_authority: solana_program::pubkey::Pubkey::from_str_const(&payer_keypair.pubkey().to_string()), // ✅ Ensure this is the payer
//         payer: solana_program::pubkey::Pubkey::from_str_const(&payer_keypair.pubkey().to_string()),
//         metadata: metadata_pda,
//         token_program: solana_program::pubkey::Pubkey::from_str_const(&spl_token::ID.to_string()),
//         system_program: solana_program::system_program::ID,
//         rent: Some(solana_program::sysvar::rent::ID),
//     }
//     .instruction(CreateMasterEditionV3InstructionArgs { max_supply: None }).data,
// };


// let transaction = anchor_client::solana_sdk::transaction::Transaction::new_signed_with_payer(
//     &[
//         create_mint_ix,
//         initialize_mint_ix,
//         create_token_account_ix,
//         mint_nft_ix,
//         create_metadata_ix.clone(),
//         create_master_edition_ix.clone(),
//     ],
//     Some(&payer_keypair.pubkey()),
//     &[payer_keypair, mint_keypair],
//     program.rpc().get_latest_blockhash().unwrap(),
// );
    
    

//     // Send and Confirm Transaction
//     let signature = program.rpc().send_and_confirm_transaction(&transaction)?;
//     println!("✅ NFT Minted! Signature: {}", signature);

//     Ok(())
// }

// pub async fn mint_nft(
//     client: &Client<Rc<&Arc<Keypair>>>,
//     payer_keypair: &Keypair,
//     mint_pubkey: Pubkey,
//     program_id: Pubkey,
//     uri: String,
//     title: String,
//     creator_key: Pubkey,
// ) {
//     let program = client.program(program_id).unwrap();

//     let metadata_pubkey = Pubkey::find_program_address(
//         &[b"metadata", &METADATA_PROGRAM_ID.to_bytes(), &mint_pubkey.to_bytes()],
//         &METADATA_PROGRAM_ID,
//     )
//     .0;

//     let master_edition_pubkey = Pubkey::find_program_address(
//         &[
//             b"metadata",
//             &METADATA_PROGRAM_ID.to_bytes(),
//             &mint_pubkey.to_bytes(),
//             b"edition",
//         ],
//         &METADATA_PROGRAM_ID,
//     )
//     .0;

//     let associated_token_account = get_associated_token_address(&payer_keypair.pubkey(), &mint_pubkey);

//     let ix = anchor_lang::solana_program::instruction::Instruction {
//         program_id,
//         accounts: MintNFT {
//             mint: mint_pubkey,
//             metadata: metadata_pubkey,
//             master_edition: master_edition_pubkey,
//             payer: payer_keypair.pubkey(),
//             mint_authority: payer_keypair.pubkey(),
//             system_program: system_program::ID,
//             rent: sysvar::rent::ID,
//             token_program: spl_token::ID,
//             token_metadata_program: METADATA_PROGRAM_ID,
//             token_account: associated_token_account,
//         }
//         .to_account_metas(None),
//         data: book_cover::instruction::MintNft {
//             creator_key,
//             uri,
//             title,
//         }
//         .data(),
//     };

//     let blockhash = program.rpc().get_latest_blockhash().unwrap();
//     let tx = Transaction::new_signed_with_payer(
//         &[ix],
//         Some(&payer_keypair.pubkey()),
//         &[payer_keypair],
//         blockhash,
//     );

//     let signature = program.rpc().send_and_confirm_transaction(&tx).unwrap();
//     println!("✅ NFT Minted Successfully! Tx Signature: {:?}", signature);
// }
