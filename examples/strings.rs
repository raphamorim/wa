use wa::string::*;

fn main() {
    println!(
        "camel_case: 'Rio de Janeiro' to {}",
        camel_case("Rio de Janeiro".to_string())
    ); // rioDeJaneiro
    println!(
        "kebab_case: 'São Paulo' to {}",
        kebab_case("São Paulo".to_string())
    ); // sao-paulo
}
