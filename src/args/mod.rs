use clap::Parser;

/// Programa utilizado para realizar busca de
/// registros DNS para um determinado dominio.
#[derive(Parser, Debug)]
#[command(
    version = "1.0", 
    about = "Sdns - Software de busca de registro de dominios.", 
    long_about = "Easy way to find DNS records for domain.",
    author = "João Victor."
)]
pub struct Args 
{
    
    /// Find NS records for domain.
    #[arg(short = 'n', long)]
    pub name_server: Option<String>,
    
    /// Find TXT records for domain. 
    #[arg(short = 't', long)]
    pub txt: Option<String>,

    /// Find MX records for domain. 
    #[arg(short = 'm', long)]
    pub mx: Option<String>,
    
    /// Find IPV4 record for domain.
    #[arg(short = 'i', long)]
    pub ipv4: Option<String>,

    #[arg()]
    pub domain: Option<String>,

}
