use std::{time::Instant, vec};
use args::Args;
use clap::Parser;
mod sdns;
mod args;

fn main() {
    let args: Args = Args::parse();
    let start: Instant = Instant::now();
    let args_actiosn: Vec<(Option<&String>, fn(&String))> = vec![
        (args.name_server.as_ref(), execute_ns_lookup),
        (args.ipv4.as_ref(), execute_ipv4_lookup),
        (args.txt.as_ref(), execute_txt_lookup),
        (args.mx.as_ref(), execute_mx_lookup),

        (args.domain.as_ref(), execute_ipv4_lookup),
        (args.domain.as_ref(), execute_ns_lookup),
        (args.domain.as_ref(), execute_mx_lookup),
        (args.domain.as_ref(), execute_txt_lookup),
    ];
    let mut _executed = false;
    for (arg, action) in args_actiosn 
    { 
        if let Some(value) = arg
        {
            action(&value);
            _executed = true;
        }
    }
    execute_time(start);
}
fn execute_ns_lookup(domain: &String)
{
    sdns::find_ns_register
    ::execute(domain);
}
fn execute_txt_lookup(domain: &String)
{
    sdns::find_txt_register
    ::execute(domain);
}

fn execute_mx_lookup(domain: &String)
{
    sdns::find_mx_register
    ::execute(domain);
}
fn execute_ipv4_lookup(domain: &String) 
{
    sdns::find_ipv4_register
    ::execute(domain);
}
fn execute_time(start: Instant) {
    println!("Executado em: [{}ms]", 
    (start.elapsed().as_secs() + (start.elapsed().subsec_nanos() / 1_000_000) 
    as u64));
}