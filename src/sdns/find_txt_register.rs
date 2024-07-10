use tokio::runtime::Runtime;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, error::ResolveError, lookup::TxtLookup, TokioAsyncResolver};

pub fn execute(dominio: &String) -> bool {
    let runtime: Runtime = Runtime::new().unwrap();
    let resolver = runtime.block_on(async {
        TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default())
    });

    let future_txt = resolver.txt_lookup(dominio);
    let response: Result<TxtLookup, ResolveError> = runtime.block_on(future_txt);
    match response { 
        Ok(txt) => {
            for txt_value in txt.as_lookup().records() {
                print!("[TXT] => ");
                if let Some(txt) = txt_value.clone().into_data() {
                    println!("{:?}", txt.as_txt().unwrap().to_string())
                }
            }
            return true;
        },
        Err(_e) => {
            println!("Endereço de dominio não possui endereço TXT");
            return false;
        }

    }
} 
