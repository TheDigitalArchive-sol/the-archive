use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};

#[test]
fn test_initialize() {
    let program_id = "FnopRi7du13hHM4b6K9kUeZwguqgznHUSaYGpPjg3sqJ";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let tx = program
        .request()
        .accounts(hogwarts_lib_rs::accounts::Initialize {})
        .args(hogwarts_lib_rs::instruction::Initialize {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
