/* As funçoes sao blocos de codigos que podem ser reutilizaveis
declaras por fn e expressadas em um bloco sem o (;) */

// SOMA DE DOIS NUMEROS INTERIOS DE 32 BITS

fn somar(a: i32, b: i32) -> i32{
    a + b   
}
    // sem o ";" -> isso é i ma EXPRESSAO, vira o retorno da funçao
    // se "a + b" (com ponto e virgula), viraria um STATEMENT e a funçao retornaria
    // "()" dando erro 

fn main() {
    let resultado = somar(4, 6):
    println!("Resultado: {}", resultado); // Resultado: 10
}
