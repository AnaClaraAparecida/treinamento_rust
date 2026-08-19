// TOPICO 1.

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


// TOPICO 2.

/* aprofundando o entendimento, a EXPRESSAO produz um valor, e o STATEMENT executa uma ação,
nao retorna valor, e tambem reforça tipagem explicita de parametros e retorno */

// RETORNA UM BOOL -> EXP. CONDICIOANL SEM "IF/ELSE" COM CHAVE E ";"

fn par(n: i32) -> bool {
    n % 2 == 0 
}

// expressao: resultado da comparaçao é o retorno

fn main() {
    let numero = 7;

    if par(numero) {
        println!("{} é par", numero);
    } else {
        println!("{} é impar", numero); // esse branch vai ser executado
    }
}

// par(numero) é chamada de funçao, o valor booleane retornado é usado diretamente na cond do "if"


// TOPICO 3.

/* uma funçao recursiva chama a si mesma até atingir um caso base, que interrompe a recursao,
Sem caso base, o programa entra em recursao infinita (stack overflow) */

// CALCULA O FATORIAL DE "n" (n!)
fn fatorial(n: u64) -> u64 {
    // CASO BASE: interrompe a recursao quando n chega a 0 
    if n == 0 {
        1
    } else {
        // CHAMADA RECURSIVA: a funçao se chama com um valor menor (n - 1)
        // ate que, em algum momento, atinja o caso base
        n * fatorial(n - 1)
    }
}

fn main() {
    // fatorial(5) = 5 * 4 * 3 * 2 * 1 * fatorial(0) = 120
    println!("5! = {}", fatorial(5)); // 5! = 120
}