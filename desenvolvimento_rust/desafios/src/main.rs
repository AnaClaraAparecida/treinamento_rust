// DESAFIO 1... TRANSAÇOES BANCARIAS 

use std::io;

fn main() {
  
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();


    if parts.len() == 3{
      let saldo: i32 = parts[0].parse::<i32>().unwrap();
      let operacao: &str = parts[1];
      let valor: i32 = parts[2].parse::<i32>().unwrap();

      match operacao {
          "deposit" => { println!("{}", saldo + valor); },
          "withdraw" => {
              if saldo >= valor {
                  println!("{}", saldo - valor);
              } else {
                  println!("Insufficient funds");
              }
          },
          _ => { println!("Operação inválida"); }
      }
    }
}
