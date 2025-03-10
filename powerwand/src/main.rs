use anchor_client::solana_sdk::{
    signature::Keypair,
    signer::{EncodableKey, SeedDerivable, Signer},
};
use anchor_client::{Client, Cluster};
use anchor_lang::prelude::Pubkey;
use anchor_spl::token::spl_token;
use dotenv::dotenv;
use light_writer_rs::{create_book_template_from_env, light_msg_decryption, light_msg_encryption};
use std::{env, error::Error, rc::Rc, str::FromStr, sync::Arc, time::Duration};
use storage_manager::{
    check_storage_account, create_storage_account, retrieve_stored_data, store_data_in_chunks,
};

mod nft_manager;
use nft_manager::{ mint_nft};
use tokio::time::sleep;
mod storage_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect("Failed to load .env file");

    let genesis_mnemonic_path = format!(
        "{}{}",
        env::var("HOME").expect("HOME environment variable not set"),
        env::var("GENESIS_KEYPAIR_PATH").expect("GENESIS_KEYPAIR_PATH not set")
    );

    let genesis_keypair = Arc::new(Keypair::read_from_file(&genesis_mnemonic_path).expect("Failed to generate Keypair from seed"));
    let client: Client<Rc<&Arc<Keypair>>> =Client::new(Cluster::Localnet, Rc::new(&genesis_keypair));
    let genesis_pubkey = genesis_keypair.pubkey();
    println!("genesis_pubkey: {:?}", genesis_pubkey);

    let env_file_path = "../light-writer-rs/books-templates/example_book_data.env";
    let json_path = "../light-writer-rs/books-templates/simulated_book_chapter.json";
    let raw_book_path = "../light-writer-rs/books-templates/simulated_book_chapter.txt";

    create_book_template_from_env(env_file_path, json_path, raw_book_path);

    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set");
    let json_file_path = "../light-writer-rs/books-templates/simulated_book_chapter.json";

    let data = light_msg_encryption(&key, json_file_path).expect("Failed to encrypt data"); // Encrypted Data (Level 22)
    println!("🗜️ Compressed data size: {} bytes", data.len());

    let sb_program_id = Pubkey::from_str(
        &env::var("BOOK_STORAGE_PROGRAM_ID").expect("BOOK_STORAGE_PROGRAM_ID not set"),
    )
    .unwrap();
    let cb_program_id = Pubkey::from_str(
        &env::var("BOOK_COVER_PROGRAM_ID").expect("BOOK_COVER_PROGRAM_ID not set"),
    )
    .unwrap();
    let storage_account = Keypair::new();
    let storage_acc_kp = create_storage_account(
        &client,
        &genesis_keypair,
        &storage_account,
        sb_program_id,
        &data,
    )
    .await;
    let _a = check_storage_account(&client, sb_program_id, storage_acc_kp)
        .await
        .unwrap();
    let _k = store_data_in_chunks(
        &client,
        &genesis_keypair,
        storage_acc_kp,
        sb_program_id,
        &data,
    )
    .await;
    sleep(Duration::from_secs(20)).await;
    let mint_account_keypair = Keypair::new();

    // let mint_account_pubkey = create_mint_account(
    //     &client,
    //     &genesis_keypair,
    //     &mint_account_keypair,
    //     &genesis_pubkey,
    //     spl_token::ID,
    // )
    // .await;
    // println!("mint_pubkey: {:?}", mint_account_pubkey);
    
    let metaplex_program_id = Pubkey::from_str(&env::var("MPL_TOKEN_METADATA_PROGRAM_ID").expect("BOOK_COVER_PROGRAM_ID not set")).unwrap();
    println!("metaplex_program_id: {:?}", metaplex_program_id);

    let nft_owner = Keypair::from_seed_phrase_and_passphrase(&env::var("NFT_OWNER_MNEMO").expect("NFT_OWNER_MNEMO not set"), "").unwrap();
    let _ = mint_nft(
        &client,
        &genesis_keypair, // this should be the nft owner after faucet
        &mint_account_keypair,
        cb_program_id,
        "uri".to_string(),
        "The Digital Archive".to_string(),
        "SYM".to_string(),
        metaplex_program_id,
        spl_token::ID,
    ).unwrap();
    println!("NFT Owner: {:?}", nft_owner.pubkey());
    // let token_account = anchor_spl::associated_token::get_associated_token_address(&genesis_keypair.pubkey(), &mint_account_pubkey);
    // println!("token_account: {:?}", token_account);

    // // let nft = mint_nft(&client, &genesis_keypair, ).await;
    // let stored_data = retrieve_stored_data(&client, sb_program_id, storage_acc_kp)
    //     .await
    //     .unwrap();
    // let decrypted_data = light_msg_decryption(&key, stored_data).expect("Failed to decrypt data");

    // println!("📖 Decompressed data: {:?}", decrypted_data);

    Ok(())
}