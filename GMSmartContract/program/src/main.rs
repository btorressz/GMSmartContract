use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
};
use solana_sdk::commitment_config::CommitmentConfig;

// Define the same struct as in the smart contract
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub name: String,
}

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate a keypair for the client
    let payer = Keypair::new();

    // Create a new account to store the name
    let new_account = Keypair::new();
    let lamports = client.get_minimum_balance_for_rent_exemption(100).unwrap();
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        lamports,
        100,
        &payer.pubkey(),
    );

    // Name to store in the account
    let name = "Brandon".to_string();
    let instruction_data = name.as_bytes();

    // Create the instruction to call the smart contract
    let program_id = Pubkey::from_str("6kVGF876e6fdP1BxDCFNAg2utzQ4KPsvVNudf6cFMFvL").unwrap();
    let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
        program_id,
        instruction_data,
        vec![new_account.pubkey()],
    );

    // Create a transaction
    let mut transaction = Transaction::new_with_payer(
        &[create_account_ix, instruction],
        Some(&payer.pubkey()),
    );

    // Sign the transaction
    let recent_blockhash = client.get_recent_blockhash().unwrap().0;
    transaction.sign(&[&payer, &new_account], recent_blockhash);

    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction sent: {}", signature);

    // Read the stored name from the account
    let account_data = client.get_account_data(&new_account.pubkey()).unwrap();
    let greeting_account: GreetingAccount = GreetingAccount::try_from_slice(&account_data).unwrap();
    println!("Stored name: {}", greeting_account.name);
}
