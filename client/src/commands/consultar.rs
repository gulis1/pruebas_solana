use anyhow::Result;
use borsh::BorshDeserialize;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, native_token::lamports_to_sol};

use crate::TokenAccount;


pub fn subcomando_consultar(client: &RpcClient, program_key: Pubkey, seed: &str) -> Result<()> {

    let (nft_key, _) = Pubkey::find_program_address(&[seed.as_bytes()], &program_key);
    let cuenta = client.get_account(&nft_key);

    println!("Consultando cuenta {}...", nft_key);
    if let Ok(cuenta) = cuenta {
        match TokenAccount::try_from_slice(&cuenta.data) {
            Ok(cuenta) => {
                println!("Dueño del NFT: {}", cuenta.token_owner);
                if cuenta.on_sale {
                    println!("El token está a la venta por {} SOL.", lamports_to_sol(cuenta.selling_price));
                }
                else { println!("El token no está a la venta."); }
            },
            Err(_) => println!("Cuenta no válida.")
        }
    }
    else { println!("Este token está libre aun.") }
    
    Ok(())
}