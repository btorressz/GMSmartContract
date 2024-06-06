use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define a struct to store the name
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub name: String,
}

// Entry point of the program
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the account to store the name
    let account = next_account_info(accounts_iter)?;

    // Deserialize the instruction data
    let name = String::from_utf8(instruction_data.to_vec())
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Print GM message
    msg!("GM, {}!", name);

    // Serialize the name and store it in the account
    let mut greeting_account = GreetingAccount { name };
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
