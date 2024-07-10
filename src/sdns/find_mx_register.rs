use tokio::runtime::Runtime;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, error::ResolveError, lookup::MxLookup, TokioAsyncResolver};
use try_unwrap::TryUnwrapOption;

pub fn execute(domain: &String) { 
    let runtime: Runtime = Runtime::new()
        .unwrap();
    let resolver = runtime.block_on(async 
        {
            TokioAsyncResolver::tokio
            (
                ResolverConfig::default(),
                ResolverOpts::default()
            )
        }
    );

    let future_mx = resolver.mx_lookup(domain);
    let response: Result<MxLookup, ResolveError> = runtime.block_on(future_mx);
    
    match response 
    { 
        Ok(mx) => 
        {
            for mx_value in mx.as_lookup().record_iter() 
            {
                let mx = mx_value.clone().into_data();
                print!("[MX] => ");
                if let Some(value) = mx
                {
                    let mx_register = value
                    .as_mx()
                    .try_unwrap();
                    if let Some(value_register) = mx_register
                    {
                        let mx = value_register.exchange().to_string();
                        println!("{:?}", mx)
                    }
                } else {println!("Endereço de dominio não possui endereço MX");}
            }
        },
        Err(_e) => {
            println!("Endereço de dominio não possui endereço MX");
        }

    }
}
