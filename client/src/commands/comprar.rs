use anyhow::{Result, Context};
use borsh::BorshDeserialize;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::instruction::{Instruction, AccountMeta};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_program;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::TokenAccount;

pub fn subcomando_comprar(client: &RpcClient, program_key: Pubkey, wallet: &Keypair, seed: &str) -> Result<()> {

    let (nft_key, _) = Pubkey::find_program_address(&[seed.as_bytes()], &program_key);
    
    let cuenta = client.get_account(&nft_key).context("No se pudo encontrar la cuena del token.")?;
    let cuenta_nft = TokenAccount::try_from_slice(&cuenta.data).context("No se pudo deserializar la información del token.")?;

    if !cuenta_nft.on_sale {
        println!("El token no está en venta.");
        return Ok(());
    }

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(wallet.pubkey(), true),
        AccountMeta::new(nft_key, false),
        AccountMeta::new(cuenta_nft.token_owner, false),
        AccountMeta::new_readonly(system_program::ID, false)
    ];
    
    let instruction_data = "comprar".to_string();
    let instruction = Instruction::new_with_bytes(program_key, instruction_data.as_bytes(), accounts);

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&wallet.pubkey()));
    let blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&wallet], blockhash);
    let result = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaccion completada con ID: {}", result);
    
    Ok(())
}