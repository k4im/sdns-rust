use tokio::runtime::Runtime;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, error::ResolveError, lookup::TxtLookup, TokioAsyncResolver};
use try_unwrap::TryUnwrapOption;

pub fn execute(domain: &String) {
    let runtime: Runtime = Runtime::new().unwrap();
    let resolver = runtime.block_on(async 
    {
        TokioAsyncResolver::tokio
        (
            ResolverConfig::default(),
            ResolverOpts::default()
        )
    });

    let future_txt = resolver.txt_lookup(domain);
    let response: Result<TxtLookup, ResolveError> = runtime.block_on(future_txt);
    match response
    {
        Ok(txt_value) => 
        {
            for txt_record in txt_value.as_lookup().record_iter()
            {
                print!("[TXT] => ");
                if let Some(record) = txt_record.clone().into_data()
                {
                    let has_record = record
                    .as_txt()
                    .try_unwrap();

                    match has_record 
                    {
                        Some(record) => 
                        {
                            println!("{:?}", record.to_string())
                        },
                        None => {
                            println!("{:?}, has no txt records", domain)
                        }    
                    }
                }
            }
            print!("\n")
        },
        Err(err) => 
        {
            if err.kind().to_string().contains("no record found for Query")
            {
                println!("[{:?}], does not contains TXT records", domain);
                return
            }
            println!("An internal error has ocurred: {:#?}", err.kind().to_string())

        }
    }
    
} 
