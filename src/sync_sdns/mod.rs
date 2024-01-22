use trust_dns_resolver::lookup::{TxtLookup, MxLookup, NsLookup};
use trust_dns_resolver::lookup_ip::LookupIp;
use trust_dns_resolver::Resolver;


pub fn buscar_registros_ipv4(resolver: &Resolver, dominio: &String) {
    let response_ip_4: Option<LookupIp> = resolver.lookup_ip(dominio).into_iter().next();
    if let Some(addr_ip) = response_ip_4 {
        for rdata in addr_ip.as_lookup().record_iter() {
            print!("[IP]: ");
            if let Some(ip) = rdata.clone().into_data() {
                println!("{:?}", ip.as_a().unwrap().0)
            }
        }
    }
}

pub fn buscar_registros_mx(resolver: &Resolver, dominio: &String) {
    let response_mx: Option<MxLookup>  = resolver.mx_lookup(dominio).into_iter().next();
    if let Some(addr_mx) = response_mx {
        for rdata in addr_mx.as_lookup().record_iter() {
            print!("[MX]:"); 
            if let Some(mx) = rdata.clone().into_data() {
                println!("{:?}", mx.as_mx().unwrap().exchange().to_string());
            }
        }
    }
}

pub fn buscar_registros_ns(resolver: &Resolver, dominio: &String) {
    let response_ns: Option<NsLookup> = resolver.ns_lookup(dominio).into_iter().next();
    if let Some(addr_ns) = response_ns { 
        for rdata in addr_ns.as_lookup().record_iter() {
            print!("[NS]: ");
                if let Some(ns_result) = rdata.clone().into_data() {
                    println!("{:?}", ns_result.as_ns().unwrap().to_string());

                }

        }
    }
}

pub fn buscar_registros_txt(resolver: &Resolver, dominio: &String) {
    let response_txt: Option<TxtLookup> = resolver.txt_lookup(dominio).into_iter().next();
    if let Some(txt_list) =  response_txt {
        for rdata in txt_list.as_lookup().record_iter() {
            print!("[TXT]: ");
            if let Some(txt) = rdata.clone().into_data() {
                println!("{}", txt.as_txt().unwrap())
            } 
        } 
    }
}
