use trust_dns_resolver::proto::rr::rdata::opt;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::env;
use std::process::exit;
use std::time::Duration;
use std::time::Instant;
mod sync_sdns;


fn main() {
    let dominio : Vec<String> = env::args().collect();
    let start: Instant = Instant::now();
    // Efetua a criação de um novo resolvedor de dominio que será do tipo Result.
    let resolver: Resolver = Resolver::new(
        ResolverConfig::default(), 
        ResolverOpts::default()).unwrap();
    
    if dominio.len() > 1 {
        sync_sdns::buscar_registros_mx(&resolver, &dominio[1]);
        sync_sdns::buscar_registros_ns(&resolver, &dominio[1]);
        sync_sdns::buscar_registros_txt(&resolver, &dominio[1]);
        sync_sdns::buscar_registros_ipv4(&resolver, &dominio[1]);
    
        let finished:Duration = start.elapsed();
        println!("Executado em: [{}ms]", (finished.as_secs() + (finished.subsec_nanos() / 1_000_000) as u64))
    } else {
        println!("Por favor insira um dominio");
        exit(1);}
}

