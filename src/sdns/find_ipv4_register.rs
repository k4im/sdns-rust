use tokio::runtime::Runtime;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, error::ResolveError,  lookup_ip::LookupIp, TokioAsyncResolver};

pub fn execute(dominio: &String) -> bool {
    let runtime: Runtime = Runtime::new().unwrap();
    let resolver = runtime.block_on(async {
        TokioAsyncResolver::tokio(
            ResolverConfig::default(), 
        ResolverOpts::default())
    });

    let future_a = resolver.lookup_ip(dominio);
    let response: Result<LookupIp, ResolveError> = runtime.block_on(future_a);
    match response { 
        Ok(a) => {
            for a_values in a.as_lookup().records() {
                print!("[IPV4] => ");
                if let Some(a) = a_values.clone().into_data() {
                    println!("{:?}", a.as_a().unwrap().0.to_string())
                }
            }
            return true;
        },
        Err(_e) => {
            println!("Endereços de dominio não possui registros tipo A");
            return false;
        }
    }
}
