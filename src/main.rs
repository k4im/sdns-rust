use trust_dns_resolver::name_server::GenericConnector;
use trust_dns_resolver::name_server::TokioRuntimeProvider;
use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;
use std::env;
use std::future::Future;
use std::process::exit;
use std::time::Instant;
use tokio::runtime::Runtime;

mod sync_sdns;
mod async_sdns;
mod help;

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
        Some("-h") => {help::help_menu()},
        Some("--help") => {help::help_menu()},
        Some("--async") => {
            buscar_registros_async(args);
            tempo_execucao(&start)
        },
        Some("-mx") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_mx(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio");   
            }
        },
        Some("-4") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_ipv4(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio");   
            }
        },
        Some("-ns") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_ns(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio");   
            }
        },
        Some("-txt") => {
            // Verifica se o o valor de dominio é existente
            if let Some(dominio) = args.get(2) {
                sync_sdns::buscar_registros_txt(&resolver, dominio);
                tempo_execucao(&start);
            }
            else {
                println!("Por favor, insira um dominio");   
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
                println!("Por favor, insira um dominio");   
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
    let io_loop:Runtime = Runtime::new().unwrap();
    
    let resolver: AsyncResolver<GenericConnector<TokioRuntimeProvider>> = io_loop.block_on(async {
        TokioAsyncResolver::tokio(
            ResolverConfig::default(), 
        ResolverOpts::default())
    });
    
    match args.get(2).map(String::as_str) {
        None => {},
        Some("-mx") => {
            if let Some(dominio) = args.get(3) {
                async_sdns::buscar_enderecos_mx_async(&io_loop, &resolver, dominio);
            }
            else {
                println!("Por favor, insira um dominio");   
            }   
        },
        Some("-txt") => {
            if let Some(dominio) = args.get(3) {
                async_sdns::buscar_enderecos_txt_async(&io_loop, &resolver, dominio);
            }
            else {
                println!("Por favor, insira um dominio");   
            }   
        },
        Some("-ns") => {
            if let Some(dominio) = args.get(3) {
                async_sdns::buscar_enderecos_ns_async(&io_loop, &resolver, dominio);
            }
            else {
                println!("Por favor, insira um dominio");   
            }   
        },
        Some("-4") => {
            if let Some(dominio) = args.get(3) {
                async_sdns::buscar_enderecos_ipv4_async(&io_loop, &resolver, dominio);
            }
            else {
                println!("Por favor, insira um dominio");   
            }   
        },
        Some(_) => {
            if let Some(dominio) = args.get(2) {
                async_sdns::buscar_enderecos_mx_async(&io_loop, &resolver, dominio);
                println!("");
                async_sdns::buscar_enderecos_ns_async(&io_loop, &resolver, dominio);
                println!("");
                async_sdns::buscar_enderecos_ipv4_async(&io_loop, &resolver, dominio);
                println!("");
                async_sdns::buscar_enderecos_txt_async(&io_loop, &resolver, dominio);            }
            else {
                println!("Por favor, insira um dominio");   
            }  
        }
    }
}

fn tempo_execucao(start : &Instant) {
    let finished = start.elapsed();
    println!("");
    println!("Executado em: [{}ms]", (finished.as_secs() + (finished.subsec_nanos() / 1_000_000) as u64));
}