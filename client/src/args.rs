use argh::FromArgs;


#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Subcomandos {
    Emitir(SubcomandoEmitir),
    Consultar(SubcomandoConsultar),
    Vender(SubcomandoVender),
    Comprar(SubcomandoComprar)
}

#[derive(FromArgs, PartialEq, Debug)]
/// Emitir un nuevo NFT
#[argh(subcommand, name = "emitir")]
pub struct SubcomandoEmitir {
    #[argh(positional)]
    /// sha256 del nft a emitir.
    pub token: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Consultar el propiertario de un NFT
#[argh(subcommand, name = "consultar")]
pub struct SubcomandoConsultar {
    #[argh(positional)]
    /// sha256 del nft a emitir.
    pub token: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Poner a la venta un NFT propio
#[argh(subcommand, name = "vender")]
pub struct SubcomandoVender {
    #[argh(positional)]
    /// token que se quiere poner a la venta
    pub token: String,

    #[argh(positional)]
    /// precio al que se quiere vender, en lamports
    pub precio: u64,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Comprar un NFT que esté a la venta
#[argh(subcommand, name = "comprar")]
pub struct SubcomandoComprar {
    #[argh(positional)]
    /// token que se quiere comprar
    pub token: String,
}

#[derive(FromArgs)]
/// Argumentos del programa
pub struct Argumentos {
    #[argh(subcommand)]
    pub subcomando: Subcomandos,
}