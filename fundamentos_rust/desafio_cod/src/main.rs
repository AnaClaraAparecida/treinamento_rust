/*  DESAFIO 1

use std::io;

fn main() {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro na leitura");

    // Converte a entrada em dois inteiros positivos
    let valores: Vec<u32> = entrada
        .split_whitespace()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let saldo = valores[0];
    let valor_compra = valores[1];

    if saldo >= valor_compra{
        println!("Compra aprovada!");
    }   else{
        println!("Saldo insuficiente")
    }
    // TODO: Verifique se o saldo é suficiente e imprima a mensagem correta ("Compra aprovada" ou "Saldo insuficiente")
}
*/

// DESAFIO 2
use std::io;

fn main() {
    // Lê uma linha da entrada padrão
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        // TODO: Se houver exatamente dois elementos, imprima a mensagem personalizada.
        // Caso contrário, imprima "Invalid input."

    if parts.len() == 2{
        let nome = parts[0];
        let tipo = parts[1];
        println!("Welcome, {nome}! Your account type is {tipo}.")
    } else {
        println!("Invalid input.")
    }
    

}

}