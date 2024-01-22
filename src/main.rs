use sync_sdns::buscar_registros_ipv4;
use sync_sdns::buscar_registros_mx;
use sync_sdns::buscar_registros_ns;
use sync_sdns::buscar_registros_txt;
use trust_dns_resolver::proto::rr::rdata::opt;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::env;
use std::process::exit;
use std::time::Duration;
use std::time::Instant;
mod sync_sdns;


fn main() {

    // Efetua a criação de um novo resolvedor de dominio que será do tipo Result.
    let resolver: Resolver = Resolver::new(
        ResolverConfig::default(), 
        ResolverOpts::default()).unwrap();
        
    let args : Vec<String> = env::args().collect();
    let start: Instant = Instant::now();

    match args.get(1).map(String::as_str) {
        Some("--async") => {
            // TODO
        },
        Some("-mx") => {
            if let Some(dominio) = args.get(2) {
                buscar_registros_mx(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }
        },
        Some("-4") => {
            if let Some(dominio) = args.get(2) {
                buscar_registros_ipv4(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }
        },
        Some("-ns") => {
            if let Some(dominio) = args.get(2) {
                buscar_registros_ns(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }
        },
        Some("-txt") => {
            if let Some(dominio) = args.get(2) {
                buscar_registros_txt(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }        
        }
        Some(_) => {
            if let Some(dominio) = args.get(1) {
                buscar_registros(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }       
        }
        None => {
            println!("Por favor insira um dominio")
        }
    }
}

fn buscar_registros(resolver: &Resolver, dominio: &String) {
    buscar_registros_mx(resolver, dominio);
    println!("");
    buscar_registros_ipv4(resolver, dominio);
    println!("");
    buscar_registros_ns(resolver, dominio);
    println!("");
    buscar_registros_txt(resolver, dominio);
}

fn tempo_execucao(start : &Instant) {
    let finished = start.elapsed();
    println!("");
    println!("Executado em: [{}ms]", (finished.as_secs() + (finished.subsec_nanos() / 1_000_000) as u64));
}