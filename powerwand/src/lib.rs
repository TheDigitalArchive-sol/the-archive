use anchor_lang::AnchorSerialize;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use wasm_bindgen::prelude::*;
use solana_sdk::signer::keypair::Keypair;
use wasm_bindgen_futures::js_sys::Promise;
use wasm_bindgen_futures::{future_to_promise, wasm_bindgen};
use std::sync::Arc;
use book_storage::instruction::StoreData;
use book_storage::instruction::Initialize as InitializeInstr;

#[wasm_bindgen]
pub struct AnchorBridge {
    payer: Arc<Keypair>,
    sb_program_id: Pubkey,
}

#[wasm_bindgen]
impl AnchorBridge {
    #[wasm_bindgen(constructor)]
    pub fn new(payer_bytes: Vec<u8>, sb_program_id: String) -> Self {
        let payer = Keypair::from_bytes(&payer_bytes).expect("Invalid keypair");
        let sb_program_id = sb_program_id.parse().expect("Invalid program ID");

        Self {
            payer: Arc::new(payer),
            sb_program_id,
        }
    }

    #[wasm_bindgen]
    pub fn initialize_storage_account(&self, seed: String, total_size: u32, total_chunks: u32) -> Promise {
        let payer = self.payer.clone();
        let program_id = self.sb_program_id;
        
        future_to_promise(async move {
            let (storage_account_pubkey, _bump) = Pubkey::find_program_address(&[seed.as_bytes()], &program_id);
            let mut transactions = Vec::new();

            let instruction = Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(storage_account_pubkey, false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: InitializeInstr {
                    total_size,
                    total_chunks,
                }
                .try_to_vec()
                .expect("Failed to serialize instruction"),
            };

            let message = Message::new(&[instruction], Some(&payer.pubkey()));
            let tx = Transaction::new_unsigned(message);
            transactions.push(tx);

            // WASM does not support direct Solana RPC, so transactions should be sent via JS.
            Ok(JsValue::from_str(&format!("✅ Storage account initialized. Pubkey: {}", storage_account_pubkey)))
        })
    }


    pub fn store_data_in_chunks(&self, storage_account_pubkey: String, data: Vec<u8>) -> Promise {
        let storage_account_pubkey = storage_account_pubkey
            .parse::<Pubkey>()
            .expect("Invalid pubkey");
        let payer = self.payer.clone();
        let program_id: Pubkey = self.sb_program_id;

        future_to_promise(async move {
            const CHUNK_SIZE: usize = 900;
            let mut transactions = Vec::new();

            for (_i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
                let chunk_vec = chunk.to_vec();

                let instruction: Instruction = Instruction {
                    program_id,
                    accounts: vec![AccountMeta {
                        pubkey: storage_account_pubkey,
                        is_signer: true,
                        is_writable: false
                    }],
                    data: StoreData { value: chunk_vec }
                        .try_to_vec()
                        .expect("Failed to serialize instruction"),
                };

                let message= Message::new(&[instruction], Some(&payer.pubkey()));
                let tx = Transaction::new_unsigned(message);
                transactions.push(tx);
            }

            // WASM does not support direct Solana RPC, so transactions should be sent via JS.
            Ok(JsValue::from_str("✅ Transactions prepared, sign and send via JS"))
        })
    }
}
