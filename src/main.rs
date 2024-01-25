use std::time::Instant;
use std::env::{self, args};
mod sdns;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start: Instant = Instant::now();
    match args.get(1).map(String::as_str){
        None => {},
        Some("-mx") => {
            if let Some(dominio) = args.get(2) {
                sdns::efetuar_busca_mx(dominio);
                tempo_execucao(start)
            }
        },
        Some("-txt") => {
            if let Some(dominio) = args.get(2) {
                sdns::efetuar_busca_txt(dominio);
                tempo_execucao(start);
            }else {
                println!("Por favor informe um dominio valido")
            }
        },
        Some("-4") => {
            if let Some(dominio) = args.get(2) {
                sdns::efetuar_busca_ipv4(dominio);
                tempo_execucao(start);
            } else {
                println!("Por favor informe um dominio valido")
            }
        },
        Some("-a") => {
            if let Some(dominio) = args.get(2) {
                sdns::efetuar_busca_ipv4(dominio);
                tempo_execucao(start);
            } else {
                println!("Por favor informe um dominio valido")
            }
        },
        Some("-ns") => {
            if let Some(dominio) = args.get(2) {
                sdns::efetuar_busca_ns(dominio);
                tempo_execucao(start);
            } else {
                println!("Por favor informe um dominio valido")
            }
        },
        Some(_) => {
            if let Some(dominio) = args.get(1) {
                sdns::efetuar_busca_ns(dominio);
                println!("");
                sdns::efetuar_busca_ipv4(dominio);
                println!("");
                sdns::efetuar_busca_txt(dominio);
                println!("");
                sdns::efetuar_busca_mx(dominio);
                println!("");
                tempo_execucao(start)
            }else {
                println!("Por favor informe um dominio valido")
            }
        },
    }

}

fn tempo_execucao(start: Instant) {
    println!("Executado em: [{}ms]", 
    (start.elapsed().as_secs() + (start.elapsed().subsec_nanos() / 1_000_000) 
    as u64));
}