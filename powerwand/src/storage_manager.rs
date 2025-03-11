// use anchor_client::Client;
// use anchor_client::solana_sdk::message::Message;
// use anchor_client::solana_sdk::transaction::Transaction;
// use anchor_client::solana_sdk::{
//     pubkey::Pubkey, signature::Keypair, signer::Signer, system_program,
// };

// use solana_sdk::client::Client;
// use anchor_client::anchor_lang::prelude::*;
// use anchor_lang::{InstructionData, ToAccountMetas};
// use solana_sdk::instruction::Instruction;
// use solana_sdk::pubkey::Pubkey;
// use solana_sdk::signature::Keypair;
// use solana_sdk::signer::Signer;
// use solana_sdk::system_program;
// use solana_sdk::transaction::Transaction;
// // use tokio::time::sleep;

// use std::rc::Rc;
// use std::sync::Arc;
// use std::thread::sleep;
// use std::time::Duration;

// pub async fn create_storage_account<T: Client>(
//     client: &T,
//     payer_keypair: &Keypair,
//     storage_account: &Keypair,
//     sb_program_id: Pubkey,
//     data: &Vec<u8>,
// ) -> Pubkey {
//     const CHUNK_SIZE: usize = 900;

//     let storage_account_pubkey: Pubkey = storage_account.pubkey();
//     println!(
//         "📦 Storage Account Public Key: {:?}",
//         storage_account_pubkey
//     );

//     let program = client.program(sb_program_id).unwrap();
//     println!("📖 Book Storage Program ID: {}", sb_program_id);

//     let total_chunks = (data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
//     println!("🪚  Predict Chunks: {}", total_chunks);

//     let required_space = 8 + 4 + 4 + (CHUNK_SIZE * total_chunks) as usize;
//     let rent_lamports = program
//         .rpc()
//         .get_minimum_balance_for_rent_exemption(required_space)
//         .unwrap();

//     println!(
//         "💰 Funding Storage Account with Rent Exemption: {}",
//         rent_lamports
//     );

//     let ix: Instruction = Instruction {
//         program_id: sb_program_id,
//         accounts: book_storage::accounts::Initialize {
//             storage_account: storage_account_pubkey,
//             user: payer_keypair.pubkey(),
//             system_program: system_program::ID,
//         }
//         .to_account_metas(None),
//         data: book_storage::instruction::Initialize {
//             total_size: CHUNK_SIZE as u32,
//             total_chunks: total_chunks as u32,
//         }
//         .data(),
//     };

//     let tx = Transaction::new_signed_with_payer(
//         &[ix],
//         Some(&payer_keypair.pubkey()),
//         &[payer_keypair, &storage_account],
//         program.rpc().get_latest_blockhash().unwrap(),
//     );

//     let signature = program.rpc().send_and_confirm_transaction(&tx).unwrap();

//     println!("✅ Storage account initialized! Signature: {:?}", signature);
//     println!("✅ Storage account ready at: {}", storage_account_pubkey);

//     storage_account_pubkey
// }

// pub async fn check_storage_account<T: Client>(
//     client: T,
//     sb_program_id: Pubkey,
//     storage_account_pubkey: Pubkey,
// ) -> bool {
//     let program = client.program(sb_program_id).unwrap();
//     match &program
//         .account::<book_storage::StorageAccount>(storage_account_pubkey)
//         .await
//         .unwrap()
//     {
//         account => {
//             println!("✅ Storage account: {} exists!", storage_account_pubkey);
//             println!("📏 Total size: {}", account.total_size);
//             println!(
//                 "📦 Total (Prediction): {} Chunks of Size: {} Total Account Size: {} ",
//                 account.total_chunks,
//                 (account.total_size / account.total_chunks),
//                 account.total_size
//             );
//             true
//         }
//         _ => {
//             println!("❌ Storage account does NOT exist.");
//             false
//         }
//     }
// }

// pub async fn retrieve_stored_data(
//     client: &Client<Rc<Arc<Keypair>>>,
//     sb_program_id: Pubkey,
//     storage_account_pubkey: Pubkey,
// ) -> Result<Vec<u8>> {
//     let program = client.program(sb_program_id).unwrap();
//     let account = program
//         .account::<book_storage::StorageAccount>(storage_account_pubkey)
//         .await
//         .unwrap();

//     println!("📥 Retrieving stored data...");
//     println!("📏 Total stored size: {} bytes", account.data.len());
//     println!("📦 Total chunks: {}", account.total_chunks);

//     if account.data.is_empty() {
//         println!("❌ No data found in storage account!");
//     }
//     Ok(account.data.clone())
// }

// pub async fn store_data_in_chunks(
//     client: &Client<Rc<Arc<Keypair>>>,
//     payer_keypair: &Keypair,
//     storage_account_pubkey: Pubkey,
//     sb_program_id: Pubkey,
//     data: &Vec<u8>,
// ) {
//     const CHUNK_SIZE: usize = 900;

//     let program = client.program(sb_program_id).unwrap();
//     let total_chunks = (data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;

//     println!(
//         "🚀 Preparing to store {} chunks in storage account {}",
//         total_chunks, storage_account_pubkey
//     );

//     let mut transactions: Vec<Transaction> = Vec::new();

//     for (i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
//         let chunk_vec = chunk.to_vec();
//         println!(
//             "📤 Preparing chunk {}/{} ({} bytes)...",
//             i + 1,
//             total_chunks,
//             chunk.len()
//         );

//         let ix = Instruction {
//             program_id: sb_program_id,
//             accounts: book_storage::accounts::StoreData {
//                 storage_account: storage_account_pubkey,
//                 user: payer_keypair.pubkey(),
//                 system_program: system_program::ID,
//             }
//             .to_account_metas(None),
//             data: book_storage::instruction::StoreData { value: chunk_vec }.data(),
//         };

//         let tx = Transaction::new_unsigned(Message::new(&[ix], Some(&payer_keypair.pubkey())));
//         transactions.push(tx);
//     }

//     let blockhash = program.rpc().get_latest_blockhash().unwrap();
//     transactions
//         .iter_mut()
//         .for_each(|tx| tx.sign(&[payer_keypair], blockhash));

//     let signatures: Vec<_> = transactions.iter().map(|tx| tx.signatures[0]).collect();
//     println!("📝 Collected signatures: {:?}", signatures);

//     for (i, tx) in transactions.iter().enumerate() {
//         sleep(Duration::from_secs(1)); // TODO: ensure order of storage!
//         match program.rpc().send_transaction(tx) {
//             Ok(signature) => println!("✅ Chunk {} stored! Tx Signature: {:?}", i + 1, signature),
//             Err(err) => println!("❌ Failed to store chunk {}: {:?}", i + 1, err),
//         }
//     }

//     println!("✅ All chunks have been sent!");
// }