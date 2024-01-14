use anyhow::Result;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::instruction::{Instruction, AccountMeta};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_program;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub fn subcomando_emitir(client: &RpcClient, program_key: Pubkey, wallet: &Keypair, seed: &str) -> Result<()> {

    let (nft_key, bump) = Pubkey::find_program_address(&[seed.as_bytes()], &program_key);
    
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(wallet.pubkey(), true),
        AccountMeta::new(nft_key, false),
        AccountMeta::new_readonly(system_program::ID, false)
    ];
    
    let instruction_data = format!("emitir,{},{}", seed, bump);
    let instruction = Instruction::new_with_bytes(program_key, instruction_data.as_bytes(), accounts);

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&wallet.pubkey()));
    let blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&wallet], blockhash);
    let result = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaccion completada con ID: {}", result);
    
    Ok(())
}