use tokio::runtime::Runtime;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, error::ResolveError, lookup::{MxLookup, NsLookup, TxtLookup}, lookup_ip::LookupIp, name_server::{GenericConnector, TokioRuntimeProvider}, AsyncResolver, TokioAsyncResolver};

pub fn efetuar_busca_mx(dominio: &String) -> bool { 
    let runtime: Runtime = Runtime::new().unwrap();
    let resolver: AsyncResolver<GenericConnector<TokioRuntimeProvider>> = runtime.block_on(async {
        TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default())
    });

    let future_mx = resolver.mx_lookup(dominio);
    let response: Result<MxLookup, ResolveError> = runtime.block_on(future_mx);
    match response { 
        Ok(mx) => {
            for mx_value in mx.as_lookup().records() {
                print!("[MX] => ");
                if let Some(mx) = mx_value.clone().into_data() {
                    println!("{:?}", mx.as_mx().unwrap().exchange().to_string())
                }
            }
            return true;
        },
        Err(_e) => {
            println!("Endereço de dominio não possui endereço MX");
            return false;
        }

    }
}

pub fn efetuar_busca_txt(dominio: &String) -> bool {
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

pub fn efetuar_busca_ns(dominio: &String) -> bool {
    let runtime: Runtime = Runtime::new().unwrap();
    let resolver = runtime.block_on(async {
        TokioAsyncResolver::tokio(
            ResolverConfig::default(), 
        ResolverOpts::default())
    });

    let future_ns = resolver.ns_lookup(dominio);
    let response: Result<NsLookup, ResolveError> = runtime.block_on(future_ns);

    match response {
        Ok(ns) => {
            for ns_value in ns.as_lookup().records() {
                print!("[NS] => ");
                if let Some(ns) = ns_value.clone().into_data() {
                    println!("{:?}", ns.as_ns().unwrap().0.to_string())
                }
            }
            return true;
        },
        Err(_e) => {
            println!("Endereço de dominio não possui registro de NS");
            return false;
        }
    }
}

pub fn efetuar_busca_ipv4(dominio: &String) -> bool {
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

#[cfg(test)]
mod test_mx_result {
    use super::*;

    #[test]
    fn nao_deve_retornar_mx() {
        let result: bool = efetuar_busca_mx(&"naoexiste.com".to_string());
        assert!(!result)
    }

    #[test]
    fn nao_deve_retornar_txt() {
        let result: bool = efetuar_busca_txt(&"naoexiste.domain".to_string());
        assert!(!result)
    }

    #[test]
    fn nao_deve_retornar_ns() {
        let result: bool = efetuar_busca_txt(&"naoexiste.domain".to_string());
        assert!(!result)
    }
}