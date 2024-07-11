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
                    let has_records = value
                    .as_mx()
                    .try_unwrap();

                    if let Some(value_register) = has_records
                    {
                        let mx = value_register
                        .exchange()
                        .to_string();
                    
                        println!("{:?}", mx)
                    }
                } else {println!("[{:?}], does not contains MX records.", domain);}
            }
            print!("\n")
        },
        Err(err) => 
        {
            if err.kind().to_string().contains("no record found for Query")
            {
                println!("[{:?}], does not contains MX records", domain);
                return
            }
            println!("An internal error has ocurred: {:#?}", err.kind().to_string())
        }

    }
}
