use anyhow::Result;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::instruction::{Instruction, AccountMeta};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub fn subcomando_vender(client: &RpcClient, program_key: Pubkey, wallet: &Keypair, seed: &str, precio: u64) -> Result<()> {

    let (nft_key, _) = Pubkey::find_program_address(&[seed.as_bytes()], &program_key);
    
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(wallet.pubkey(), true),
        AccountMeta::new(nft_key, false),
    ];
    
    let instruction_data = format!("poner_venta,{}", precio);
    let instruction = Instruction::new_with_bytes(program_key, instruction_data.as_bytes(), accounts);

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&wallet.pubkey()));
    let blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&wallet], blockhash);
    let result = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaccion completada con ID: {}", result);
    
    Ok(())
}