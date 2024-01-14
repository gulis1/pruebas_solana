mod args;
mod commands;

use std::str::FromStr;
use anyhow::Result;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKey, Signer};
use borsh::{BorshDeserialize, BorshSerialize};


use crate::args::{Argumentos, Subcomandos};
use crate::commands::comprar::subcomando_comprar;
use crate::commands::consultar::subcomando_consultar;
use crate::commands::emitir::subcomando_emitir;
use crate::commands::poner_venta::subcomando_vender;

const PROGRAM_ID: &str = "7UtzoFgY5tqMia8fh6ARnbiZ3dVWeRu9JcrfzVucoVFz";
const WALLET_PATH: &str = "/home/gulis/.config/solana/id.json";
const RPC_URL: &str = "http://127.0.0.1:8899";


#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenAccount {
    pub token_owner: Pubkey,
    pub on_sale: bool,
    pub selling_price: u64
}


fn main() -> Result<()> {

    let args: Argumentos = argh::from_env();
    let client = RpcClient::new(String::from(RPC_URL));
    
    let wallet = Keypair::read_from_file(WALLET_PATH).expect("Failed to load wallet info");
    println!("Pubkey del wallet: {}", wallet.pubkey());
    let program_key = Pubkey::from_str(PROGRAM_ID)?;
    
    match args.subcomando {
        Subcomandos::Emitir(subcmd) => subcomando_emitir(&client, program_key, &wallet, &subcmd.token),
        Subcomandos::Consultar(subcmd) => subcomando_consultar(&client, program_key, &subcmd.token) ,
        Subcomandos::Vender(subcmd) => subcomando_vender(&client, program_key, &wallet, &subcmd.token, subcmd.precio),
        Subcomandos::Comprar(subcmd) => subcomando_comprar(&client, program_key, &wallet, &subcmd.token)
    }?;

    Ok(())
}
