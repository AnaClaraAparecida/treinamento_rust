const TIPO_DE_DADOS:i8 = 2; // -> as constantes sao imutavies 
static mut UMA_VARIAVEL_ESTATICA: i8 = 3; // -> ja as static, podem ser mutaveis, so colocar o mut logo depois, mas precisa do unsafe

fn main() {
    unsafe{ // -> ele serve para fazer mudanças somente de Static, pois as costantes sao imutaveis, mesmo que haja formaçao mut na static, é necessario que tenha o unsafe
        UMA_VARIAVEL_ESTATICA = 4;
        println!("Constante: {}", TIPO_DE_DADOS);
        println!("Statica: {}", UMA_VARIAVEL_ESTATICA);
    }
    imprime();
}

fn imprime() {
    unsafe{
        UMA_VARIAVEL_ESTATICA = 4;
        println!("Constante: {}", TIPO_DE_DADOS);
        println!("Statica: {}", UMA_VARIAVEL_ESTATICA);
    }

}
