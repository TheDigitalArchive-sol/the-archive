use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::{Promise, Uint8Array};
use wasm_bindgen_futures::{future_to_promise, wasm_bindgen};
use serde_wasm_bindgen::to_value;
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

        let (storage_account_pubkey, _bump) = Pubkey::find_program_address(&[seed.as_bytes()], &program_id); 
        let mut instr_data = vec![];
        let discriminator = solana_sdk::hash::hash(b"global:initialize").to_bytes();
        
        instr_data.extend_from_slice(&discriminator[..8]); // First 8 bytes = instruction discriminator
        instr_data.extend_from_slice(&(seed.len() as u32).to_le_bytes()); // Encode seed length
        instr_data.extend_from_slice(seed.as_bytes()); // Encode seed
        instr_data.extend_from_slice(&total_size.to_le_bytes()); // Encode total_size
        instr_data.extend_from_slice(&total_chunks.to_le_bytes()); // Encode total_chunks

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

    #[wasm_bindgen]
    pub fn store_data_in_chunks(&self, storage_account_pubkey: String, data: Vec<u8>, chunk_size: usize) -> Promise {
        let storage_account_pubkey = storage_account_pubkey.parse::<Pubkey>().expect("Invalid pubkey");
        let payer = self.payer_pubkey.clone();
        let program_id: Pubkey = self.sb_program_id;
    
        future_to_promise(async move {
            let mut transactions_base64 = Vec::new();
    
            for chunk in data.chunks(chunk_size) {
                let chunk_vec = chunk.to_vec();
    
                let mut instr_data = vec![];
                let discriminator = solana_sdk::hash::hash(b"global:store_data").to_bytes();
                instr_data.extend_from_slice(&discriminator[..8]);
                instr_data.extend_from_slice(&(chunk_vec.len() as u32).to_le_bytes());
                instr_data.extend_from_slice(&chunk_vec);
    
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
                let tx = Transaction::new_unsigned(message);
    
                let tx_bytes = tx.message_data(); 
                let tx_base64 = base64::encode(tx_bytes);
                transactions_base64.push(tx_base64);
            }
    
            Ok(to_value(&transactions_base64).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)})
    }


    #[wasm_bindgen]
    pub fn light_msg_encryption(&self, key: &str, rsd: &str) -> Result<Uint8Array, JsValue> {
        eprintln!("🔍 Debug: Received key: {:?}", key);
        eprintln!("🔍 Debug: Received JSON: {:?}", rsd);
    
        let data = light_writer_rs::light_msg_encryption(key, rsd)
            .map_err(|err| JsValue::from_str(&format!("Encryption error: {}", err)))?;
    
        eprintln!("✅ Encryption successful!");
        Ok(Uint8Array::from(&data[..]))
    }
    

    #[wasm_bindgen]
    pub fn light_msg_decryption(&self, key: &str, cbd: &[u8]) -> Result<JsValue, JsValue> {
        eprintln!("🔍 Decrypting buffer of len: {:?}", cbd.len());
    
        let data = light_writer_rs::light_msg_decryption(key, cbd.to_vec())
            .map_err(|e| JsValue::from_str(&format!("Decryption error: {}", e)))?;
    
        to_value(&data).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
    
     
}
