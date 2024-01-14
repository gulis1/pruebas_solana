use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError, system_instruction::transfer, program::invoke,
};

use crate::TokenAccount;

pub fn comprar( accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let account_token = next_account_info(accounts_iter)?;
    let seller_account = next_account_info(accounts_iter)?;
    let system_account = next_account_info(accounts_iter)?;

    let mut cuenta_nft = TokenAccount::try_from_slice(&account_token.data.borrow())?;
    
    if !cuenta_nft.on_sale {
        return Err(ProgramError::InvalidAccountData);
    }

    let instruccion_pago = transfer(
        payer_account.key,
        seller_account.key, 
        cuenta_nft.selling_price
    );

    invoke(
        &instruccion_pago,
        &[payer_account.clone(), seller_account.clone(), system_account.clone()],
    )?;

    cuenta_nft.on_sale = false;
    cuenta_nft.selling_price = 0;
    cuenta_nft.token_owner = *payer_account.key;
    cuenta_nft.serialize(&mut *account_token.data.borrow_mut())?;


    msg!("Token comprado correctamente");
    Ok(())
}