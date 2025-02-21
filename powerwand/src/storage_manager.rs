use anchor_client::solana_sdk::{pubkey::Pubkey, system_program, signature::Keypair, signer::Signer};
use anchor_client::solana_sdk::transaction::Transaction;
use anchor_client::Client;

use anchor_lang::{ToAccountMetas, InstructionData};
use anchor_client::anchor_lang::prelude::*;

use std::sync::Arc;
use std::rc::Rc;

pub async fn create_storage_account(
    client: &Client<Rc<&Arc<Keypair>>>,
    payer_keypair: &Keypair,
    storage_account: &Keypair,
    sb_program_id: Pubkey,
    data: &Vec<u8>,
) -> Pubkey {
    const CHUNK_SIZE: usize = 900;
    
    let storage_account_pubkey = storage_account.pubkey();
    println!("📦 Storage Account Public Key: {:?}", storage_account_pubkey);

    let program = client.program(sb_program_id).unwrap();
    println!("📖 Book Storage Program ID: {}", sb_program_id);

    let total_chunks = (data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
    println!("🪚  Predict Chunks: {}", total_chunks);

    let required_space = 8 + 4 + 4 + (CHUNK_SIZE * total_chunks) as usize;
    let rent_lamports = program.rpc().get_minimum_balance_for_rent_exemption(required_space).unwrap();

    println!("💰 Funding Storage Account with Rent Exemption: {}", rent_lamports);

    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: sb_program_id,
        accounts: book_storage::accounts::Initialize {
            storage_account: storage_account_pubkey,
            user: payer_keypair.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: book_storage::instruction::Initialize {
            total_size: CHUNK_SIZE as u32,
            total_chunks: total_chunks as u32,
        }
        .data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer_keypair.pubkey()),
        &[payer_keypair, &storage_account],
        program.rpc().get_latest_blockhash().unwrap(),
    );

    let signature = program.rpc().send_and_confirm_transaction(&tx).unwrap();

    println!("✅ Storage account initialized! Signature: {:?}", signature);
    println!("✅ Storage account ready at: {}", storage_account_pubkey);

    storage_account_pubkey
}


pub async fn check_storage_account(
    client: &Client<Rc<&Arc<Keypair>>>,
    sb_program_id: Pubkey,
    storage_account_pubkey: Pubkey,
) -> Result<bool> {
    let program = client.program(sb_program_id).unwrap();
    match &program.account::<book_storage::StorageAccount>(storage_account_pubkey).await.unwrap() {
        account => {
            println!("✅ Storage account: {} exists!", storage_account_pubkey);
            println!("📏 Total size: {}", account.total_size);
            println!("📦 Total (Prediction): {} Chunks of Size: {} Total Account Size: {} ", account.total_chunks, (account.total_size/account.total_chunks), account.total_size);
            Ok(true)
        }
        _ => {
            println!("❌ Storage account does NOT exist.");
            Ok(false)
        }
    }
}

pub async fn retrieve_stored_data(
    client: &Client<Rc<&Arc<Keypair>>>,
    sb_program_id: Pubkey,
    storage_account_pubkey: Pubkey,
) -> Result<Vec<u8>> {
    let program = client.program(sb_program_id).unwrap();
    let account = program.account::<book_storage::StorageAccount>(storage_account_pubkey).await.unwrap();

    println!("📥 Retrieving stored data...");
    println!("📏 Total stored size: {} bytes", account.data.len());
    println!("📦 Total chunks: {}", account.total_chunks);

    if account.data.is_empty() {
        println!("❌ No data found in storage account!");
    }
    Ok(account.data.clone())
}

pub async fn store_data_in_chunks(
    client: &Client<Rc<&Arc<Keypair>>>,
    payer_keypair: &Keypair,
    storage_account_pubkey: Pubkey,
    sb_program_id: Pubkey,
    data: &Vec<u8>,
) {
    const CHUNK_SIZE: usize = 900;

    let program = client.program(sb_program_id).unwrap();
    let total_chunks = (data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;

    println!("🚀 Storing {} chunks in storage account {}", total_chunks, storage_account_pubkey);

    for (i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
        let chunk_vec = chunk.to_vec();
        println!("📤 Sending chunk {}/{} ({} bytes)...", i + 1, total_chunks, chunk.len());

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: sb_program_id,
            accounts: book_storage::accounts::StoreData {
                storage_account: storage_account_pubkey,
                user: payer_keypair.pubkey(),
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: book_storage::instruction::StoreData { value: chunk_vec }.data(),
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer_keypair.pubkey()),
            &[payer_keypair],
            program.rpc().get_latest_blockhash().unwrap(),
        );

        match program.rpc().send_and_confirm_transaction(&tx) {
            Ok(signature) => println!("✅ Chunk {} stored! Tx Signature: {:?}", i + 1, signature),
            Err(err) => {
                println!("❌ Failed to store chunk {}: {:?}", i + 1, err);
                break;
            }
        }
    }

    println!("✅ All chunks have been sent!");
}
