fn main() {
    /*
    Ano de nascimento, subtração pelo ano atual,
    deve ter a devoluçao da idade da pessoa em questao 
    */

    let ano_nsc: i16 = 2007;
    let ano_atual: i16 = 2026;

    let idade: i16 = ano_atual - ano_nsc;  
    
    println!("a idade calculada para o ano de {} é {}", ano_nsc, idade);
}
