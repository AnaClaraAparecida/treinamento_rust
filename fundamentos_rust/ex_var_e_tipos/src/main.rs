fn main() {
    /*
    Ano de nascimento, subtração pelo ano atual,
    deve ter a devoluçao da idade da pessoa em questao 
    */

    let nome: &str = "Ana";

    let ano_nsc: u16= 2007;
    let ano_atual:u16 = 2026;

    let idade: u16 = ano_atual - ano_nsc;  
    
    println!("a idade da {} calculada para o ano de {} é {} anos", nome, ano_nsc, idade);
}
