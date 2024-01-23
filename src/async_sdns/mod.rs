use trust_dns_resolver::name_server::GenericConnector;
use trust_dns_resolver::name_server::TokioRuntimeProvider;
use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;
use tokio::runtime::Runtime;
use trust_dns_resolver::lookup::{TxtLookup, MxLookup, NsLookup};
use trust_dns_resolver::lookup_ip::LookupIp;

pub fn buscar_enderecos_ipv4_async(runtime: &Runtime, 
    resolver: &AsyncResolver<GenericConnector<TokioRuntimeProvider>>, 
    args: &String) {
    
    let lookup_future = resolver.lookup_ip(args);
    let response = runtime.block_on(lookup_future).into_iter().next();
    if let Some(ip) = response {
        for ip_addr in ip.as_lookup().records() {
            print!("[IP]: ");
            if let Some(ip) = ip_addr.clone().into_data() {
                println!("{:?}", ip.as_a().unwrap().0)
            }
        }
    }

}   

pub fn buscar_enderecos_mx_async(runtime: &Runtime, 
    resolver: &AsyncResolver<GenericConnector<TokioRuntimeProvider>>, 
    args: &String) {
    
    let lookup_future_mx = resolver.mx_lookup(args);
    let response = runtime.block_on(lookup_future_mx).into_iter().next();
    if let Some(mx) = response {
        for mx_addr in mx.as_lookup().records() {
            print!("[MX]: ");
            if let Some(mx_record) = mx_addr.clone().into_data() {
                println!("{:?}", mx_record.as_mx().unwrap().exchange().to_string());
            }
        }
    }  
}

pub fn buscar_enderecos_ns_async(runtime: &Runtime, 
    resolver: &AsyncResolver<GenericConnector<TokioRuntimeProvider>>, 
    args: &String) {
    
    let lookup_future_ns = resolver.ns_lookup(args);
    let response = runtime.block_on(lookup_future_ns).into_iter().next();
    if let Some(ns) = response {
        for ns_value in ns.as_lookup().records() {
            print!("[NS]: ");
            if let Some(ns_record) = ns_value.clone().into_data() {
                println!("{:?}", ns_record.as_ns().unwrap().0.to_string())
            }
        }
    }
}

pub fn buscar_enderecos_txt_async(runtime: &Runtime, 
    resolver: &AsyncResolver<GenericConnector<TokioRuntimeProvider>>, 
    args: &String) {
    
    let lookup_future_ns = resolver.txt_lookup(args);
    let response = runtime.block_on(lookup_future_ns).into_iter().next();
    if let Some(txt) = response {
        for txt_value in txt.as_lookup().records() {
            print!("[TXT]: ");
            if let Some(txt_record) = txt_value.clone().into_data() {
                println!("{:?}", txt_record.as_txt().unwrap().to_string())
            }
        }
    }
}