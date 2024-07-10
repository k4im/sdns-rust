pub mod find_ns_register;
pub mod find_mx_register;
pub mod find_txt_register;
pub mod find_ipv4_register;


// #[cfg(test)]
// mod test_mx_result {
//     use super::*;

//     #[test]
//     fn nao_deve_retornar_mx() {
//         let result: bool = efetuar_busca_mx(&"naoexiste.com".to_string());
//         assert!(!result)
//     }

//     #[test]
//     fn nao_deve_retornar_txt() {
//         let result: bool = efetuar_busca_txt(&"naoexiste.domain".to_string());
//         assert!(!result)
//     }

//     #[test]
//     fn nao_deve_retornar_ns() {
//         let result: bool = efetuar_busca_txt(&"naoexiste.domain".to_string());
//         assert!(!result)
//     }
// }