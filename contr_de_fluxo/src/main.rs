fn main() {
    /*
    Ano de nascimento, subtração pelo ano atual,
    deve ter a devoluçao da idade da pessoa em questao 
    */

    let nome: &str = "Ana";

    let ano_nsc: u16 = 2007;
    let mes_nsc: u16 = 10;
    let dia_nsc: u16 = 26;

    let ano_atual: u16 = 2026;
    let mes_atual: u16 = 07;
    let dia_atual: u16 = 28;

    let mut idade: u16 = ano_atual - ano_nsc;  

    if mes_nsc > mes_atual {
        idade -= 1;
    }
    
    println!("a idade da {}, calculada para o ano de {}, é {} anos", nome, ano_nsc, idade);
}