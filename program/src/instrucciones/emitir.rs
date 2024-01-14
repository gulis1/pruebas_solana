use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
};

use crate::TokenAccount;

pub fn emitir_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    arguments: &[&str],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_account = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;

    let payer_balance = payer_account.lamports();
    let rent = Rent::default();
    let space = TokenAccount::default().try_to_vec()?.len();
    let minimum_balance = rent.minimum_balance(space);

    msg!("Balance del comprador: {}", payer_balance);
    msg!("Minimo balance para sacar cuenta: {}", minimum_balance);

    if payer_balance < minimum_balance {
        return Err(ProgramError::InsufficientFunds);
    }

    let seed: String = arguments[0].into();
    let bump: u8 = arguments[1]
        .parse()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let expected_pda = Pubkey::create_program_address(&[seed.as_bytes(), &[bump]], program_id)?;
    msg!("Expected pda: {}", expected_pda.to_string());
    msg!("Received pda: {}", new_account.key.to_string());

    if &expected_pda != new_account.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let instruction_create_nft = create_account(
        payer_account.key,
        new_account.key,
        minimum_balance,
        space as u64,
        program_id,
    );

    invoke_signed(
        &instruction_create_nft,
        accounts,
        &[&[seed.as_bytes(), &[bump]]],
    )?;

    let mut cuenta_nft = TokenAccount::try_from_slice(&new_account.data.borrow())?;
    cuenta_nft.token_owner = *payer_account.key;
    cuenta_nft.serialize(&mut *new_account.data.borrow_mut())?;

    msg!(
        "Succesfully stored token on address: {}",
        new_account.key.to_string()
    );
    Ok(())
}