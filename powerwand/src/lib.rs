use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Promise;
use wasm_bindgen_futures::{future_to_promise, wasm_bindgen};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
struct InitializeInstr {
    total_size: u32,
    total_chunks: u32,
}
#[derive(BorshSerialize, BorshDeserialize)]
struct StoreData {
    value: Vec<u8>
}

#[wasm_bindgen]
pub struct AnchorBridge {
    payer_pubkey: Pubkey,
    sb_program_id: Pubkey,
}

#[wasm_bindgen]
impl AnchorBridge {
    #[wasm_bindgen(constructor)]
    pub fn new(payer_pubkey: String, sb_program_id: String) -> Self {
        let payer_pubkey = payer_pubkey.parse().expect("Invalid public key");
        let sb_program_id = sb_program_id.parse().expect("Invalid program ID");

        Self {
            payer_pubkey,
            sb_program_id,
        }
    }

    #[wasm_bindgen]
    pub fn initialize_storage_account(&self, seed: String, total_size: u32, total_chunks: u32) -> Promise {
        let payer = self.payer_pubkey.clone();
        let program_id = self.sb_program_id;
        
        future_to_promise(async move {
            let seed_bytes = seed.as_bytes();
        let (storage_account_pubkey, _bump) = Pubkey::find_program_address(&[seed_bytes], &program_id); 
            let mut instr_data = vec![];
            instr_data.extend_from_slice(&(seed.len() as u32).to_le_bytes());
            instr_data.extend_from_slice(&total_size.to_le_bytes());
            instr_data.extend_from_slice(&total_chunks.to_le_bytes());
    
        let instr = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(storage_account_pubkey, false),
                AccountMeta::new(payer, true),
                AccountMeta::new_readonly(solana_sdk::system_program::ID, false),
            ],
            data: instr_data,
        };

            let message = Message::new(&[instr], Some(&payer));
            let tx: Transaction = Transaction::new_unsigned(message);
            let tx_bytes = tx.message_data();
            let tx_base64 = base64::encode(tx_bytes);

            Ok(JsValue::from_str(&tx_base64))
        })
    }

    pub fn store_data_in_chunks(&self, storage_account_pubkey: String, data: Vec<u8>) -> Promise {
        let storage_account_pubkey = storage_account_pubkey
            .parse::<Pubkey>()
            .expect("Invalid pubkey");
        let payer = self.payer_pubkey.clone();
        let program_id: Pubkey = self.sb_program_id;

        future_to_promise(async move {
            const CHUNK_SIZE: usize = 900;
            let mut transactions = Vec::new();

            for (_i, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
                let chunk_vec = chunk.to_vec();



                let instr = Instruction::new_with_borsh(
                    program_id,
                    &StoreData { value: chunk_vec },
                    vec![
                        AccountMeta::new(storage_account_pubkey, true),
                        AccountMeta::new(payer, false),
                    ],
               );

                let message= Message::new(&[instr], Some(&payer));
                let tx = Transaction::new_unsigned(message);
                transactions.push(tx);
            }

            // WASM does not support direct Solana RPC, so transactions should be sent via JS.
            Ok(JsValue::from_str("✅ Transactions prepared, sign and send via JS"))
        })
    }
}
