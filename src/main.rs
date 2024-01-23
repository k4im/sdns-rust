use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use std::env;
use std::process::exit;
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
        None => {
            println!("Por favor insira um dominio");
            exit(1);
        },
        Some("--async") => {
            buscar_registros_async(args);
        },
        Some("-mx") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_mx(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }
        },
        Some("-4") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_ipv4(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }
        },
        Some("-ns") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_ns(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }
        },
        Some("-txt") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_txt(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }        
        }
        // Executa uma ação para todos os valores que não foram listados no Match
        Some(_) => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(1) {
                buscar_registros(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio")    
            }       
        }
    }
}

fn buscar_registros(resolver: &Resolver, dominio: &String) {
    sync_sdns::buscar_registros_mx(resolver, dominio);
    println!("");
    sync_sdns::buscar_registros_ipv4(resolver, dominio);
    println!("");
    sync_sdns::buscar_registros_ns(resolver, dominio);
    println!("");
    sync_sdns::buscar_registros_txt(resolver, dominio);
}
fn buscar_registros_async(args: Vec<String>) {
    match args.get(2).map(String::as_str) {
        None => {},
        Some("-mx") => {},
        Some("-txt") => {},
        Some("-ns") => {},
        Some("-a") => {},
        Some(_) => {
            
        }
    }
}
fn tempo_execucao(start : &Instant) {
    let finished = start.elapsed();
    println!("");
    println!("Executado em: [{}ms]", (finished.as_secs() + (finished.subsec_nanos() / 1_000_000) as u64));
}