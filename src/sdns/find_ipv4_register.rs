use tokio::runtime::Runtime;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, error::ResolveError,  lookup_ip::LookupIp, TokioAsyncResolver};
use try_unwrap::TryUnwrapOption;

pub fn execute(domain: &String) {
    let runtime: Runtime = Runtime::new().unwrap();
    let resolver = runtime.block_on(async {
        TokioAsyncResolver::tokio(
            ResolverConfig::default(), 
        ResolverOpts::default())
    });

    let future_a = resolver.lookup_ip(domain);
    let response: Result<LookupIp, ResolveError> = runtime.block_on(future_a);
    match response { 
        Ok(a) => 
        {
            for a_values in a.as_lookup().records() 
            {
                print!("[IPV4] => ");
                let has_records = a_values
                .clone()
                .into_data()
                .try_unwrap();
                
                match has_records 
                {
                    Some(record) => 
                    {
                        let ip = record
                        .as_a()
                        .try_unwrap();
                        if let Some(value) = ip 
                        {
                            let ip_addr = value.0.to_string();
                            println!("{:?}", ip_addr);
                        }
                    
                    }
                    None => 
                    {
                        println!("{:?}, has no A records", domain)
                 
                    }
                }
            }
            print!("\n");
        },
        Err(err) => 
        {
            if err.kind().to_string().contains("no record found for Query")
            {
                println!("[{:?}], does not contains A records", domain);
                return
            }
            println!("An internal error has ocurred: {:#?}", err.kind().to_string())
        }
    }
}
