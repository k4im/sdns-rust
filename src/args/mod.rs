use clap::Parser;

/// Programa utilizado para realizar busca de
/// registros DNS para um determinado dominio.
#[derive(Parser, Debug)]
#[command(
    version = "1.0", 
    about = "Sdns - Software de busca de registro de dominios.", 
    long_about = "Através deste software será possivel estar realizando a operação
    de estar efetuando a busca de registros de DNS para um determinado dominio.",
    author = "João Victor."
)]
pub struct Args 
{
    
    /// Efetua a busca de registros NS 
    /// para um determinado dominio.
    #[arg(short = 'n', long)]
    pub name_server: Option<String>,
    
    /// Efetua a busca de registros TXT 
    /// para determinado dominio.
    #[arg(short = 't', long)]
    pub txt: Option<String>,

    /// Efetua busca de registros MX 
    /// para determinado dominio.
    #[arg(short = 'm', long)]
    pub mx: Option<String>,
    
    /// Efetua a busca de registros IPV4 
    /// para determinado dominio.
    #[arg(short = 'i', long)]
    pub ipv4: Option<String>,

    #[arg()]
    pub domain: Option<String>,

}
