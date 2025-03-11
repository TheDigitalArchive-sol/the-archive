// use anchor_client::solana_sdk::{
//     signature::Keypair,
//     signer::{EncodableKey, Signer},
// };
// use anchor_client::{Client, Cluster};
// use anchor_lang::prelude::Pubkey;
// use dotenv::dotenv;
// use light_writer_rs::{create_book_template_from_env, light_msg_decryption, light_msg_encryption};
// use std::{env, error::Error, rc::Rc, str::FromStr, sync::Arc, time::Duration};
// use storage_manager::{
//     check_storage_account, create_storage_account, retrieve_stored_data, store_data_in_chunks,
// };
// use tokio::time::sleep;
// mod storage_manager;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     dotenv().expect("Failed to load .env file");

//     let genesis_mnemonic_path = format!(
//         "{}{}",
//         env::var("HOME").expect("HOME environment variable not set"),
//         env::var("GENESIS_KEYPAIR_PATH").expect("GENESIS_KEYPAIR_PATH not set")
//     );

//     let genesis_keypair = Arc::new(
//         Keypair::read_from_file(&genesis_mnemonic_path)
//             .expect("Failed to generate Keypair from seed"),
//     );
//     let client: Client<Rc<Arc<Keypair>>> =
//         Client::new(Cluster::Localnet, Rc::new(genesis_keypair.clone()));
//     let genesis_pubkey = genesis_keypair.pubkey();
//     println!("genesis_pubkey: {:?}", genesis_pubkey);

//     let env_file_path = "../light-writer-rs/books-templates/example_book_data.env";
//     let json_path = "../light-writer-rs/books-templates/simulated_book_chapter.json";
//     let raw_book_path = "../light-writer-rs/books-templates/simulated_book_chapter.txt";

//     create_book_template_from_env(env_file_path, json_path, raw_book_path);

//     let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set");
//     let json_file_path = "../light-writer-rs/books-templates/simulated_book_chapter.json";

//     let data = light_msg_encryption(&key, json_file_path).expect("Failed to encrypt data"); // Encrypted Data (Level 22)
//     println!("🗜️ Compressed data size: {} bytes", data.len());

//     let sb_program_id = Pubkey::from_str(
//         &env::var("BOOK_STORAGE_PROGRAM_ID").expect("BOOK_STORAGE_PROGRAM_ID not set"),
//     )
//     .unwrap();
//     let storage_account = Keypair::new();
//     let storage_acc_kp = create_storage_account(
//         &client,
//         &genesis_keypair,
//         &storage_account,
//         sb_program_id,
//         &data,
//     )
//     .await;
//     let _a = check_storage_account(&client, sb_program_id, storage_acc_kp)
//         .await
//         .unwrap();
//     let _k = store_data_in_chunks(
//         &client,
//         &genesis_keypair,
//         storage_acc_kp,
//         sb_program_id,
//         &data,
//     )
//     .await;
//     sleep(Duration::from_secs(20)).await;
//     let stored_data = retrieve_stored_data(&client, sb_program_id, storage_acc_kp)
//         .await
//         .unwrap();
//     let decrypted_data = light_msg_decryption(&key, stored_data).expect("Failed to decrypt data");

//     println!("📖 Decompressed data: {:?}", decrypted_data);

//     Ok(())
// }

fn main() {
    println!("Hello, world!");
}
