use tokio::runtime::Runtime;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, error::ResolveError, lookup::NsLookup, TokioAsyncResolver};

pub fn execute(domain: &String)
{
    let runtime: Runtime = Runtime::new()
    .unwrap();

    let resolver = runtime.block_on(async 
        {
            TokioAsyncResolver::tokio(
                ResolverConfig::default(),
                ResolverOpts::default() 
            )
        }
    );

    let future_ns = resolver.ns_lookup(domain);
    let response: Result<NsLookup, ResolveError> = runtime.block_on(future_ns);

    match response
    {
        Ok(ns_result) =>
        {
            for value in ns_result.as_lookup().record_iter()
            {
                print!("[NS] => ");
                if let Some(ns_value) = value.clone().data()
                {
                    let ns_name = Some(ns_value
                        .as_ns()
                        .unwrap()
                        .0.to_string()
                    );
                    match ns_name 
                    {
                        Some(ns) => 
                        {
                            println!("{:?}", ns);
                        },
                        None => {}    
                    }
                } else {println!("Dominio não possui registro NS.")}
                
            }
        }
        Err(err) => 
        {
            if err.kind().to_string().contains("no record found for Query")
            {
                println!("Domain does not contains NS");
                return
            }
            println!("{:#?}", err.kind().to_string())
        }
    }
}