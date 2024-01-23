pub fn help_menu() {
    println!("--help         Chama menu de help.");
    println!("--async        Executa sdns utilizando async.");
    println!("-mx            Retorna apenas os endereços mx de um dominio.");
    println!("-txt           Retorna apenas os valores txt de um dominio.");
    println!("-4             Retorna apenas os valores ipv4 de um dominio.");
    println!("-ns            Retorna apenas os endereços ns de um dominio.");
    println!("------------------------------------------------------------");
    println!("--async -mx    Retorna apenas os endereços mx de um dominio de forma async.");
    println!("--async -ns    Retorna apenas os endereços ns de um dominio de forma async.");
    println!("--async -4     Retorna apenas os endereços ipv4 de um dominio de forma async.");
    println!("--async -txt   Retorna apenas os endereços ipv4 de um dominio de forma async.");

}