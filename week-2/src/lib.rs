use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize)]

pub enum NameInstruction {
    Initialize(String),
    Update(String),
}

#[derive(BorshDeserialize, BorshSerialize)]

pub struct Data {
    name: String,
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
}
