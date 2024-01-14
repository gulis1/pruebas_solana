pub mod instrucciones;

use borsh::{BorshDeserialize, BorshSerialize};
use instrucciones::{emitir::emitir_token, poner_venta::poner_venta, comprar::comprar};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenAccount {
    pub token_owner: Pubkey,
    pub on_sale: bool,
    pub selling_price: u64
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let instruction_data = String::from_utf8(Vec::from(_instruction_data))
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let arguments: Vec<&str> = instruction_data.split(',').collect();
    let instrucion_type = arguments[0];

    match instrucion_type {
        "emitir" => emitir_token(program_id, accounts, &arguments[1..]),
        "poner_venta" => poner_venta(accounts, &arguments[1..]),
        "comprar" => comprar(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}