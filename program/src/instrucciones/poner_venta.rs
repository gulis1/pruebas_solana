use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
};

use crate::TokenAccount;

pub fn poner_venta(
    accounts: &[AccountInfo],
    arguments: &[&str],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let account = next_account_info(accounts_iter)?;
    let precio_lamports: u64 = arguments[0].parse().map_err(|_| ProgramError::InvalidInstructionData)?;

    let mut cuenta_nft = TokenAccount::try_from_slice(&account.data.borrow())?;
    if &cuenta_nft.token_owner != payer_account.key {
        return Err(ProgramError::InvalidAccountOwner);
    }

    cuenta_nft.on_sale = true;
    cuenta_nft.selling_price = precio_lamports;
    cuenta_nft.serialize(&mut *account.data.borrow_mut())?;

    msg!("Token puesto a la venta por: {} lamports", precio_lamports);
    Ok(())
}