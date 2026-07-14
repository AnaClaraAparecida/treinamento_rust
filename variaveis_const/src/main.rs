const TIPO_DE_DADOS: i8 = 2;

static mut UMA_VARIAVEL_ESTATICA: i8 = 3;

fn main() {
    unsafe {
        UMA_VARIAVEL_ESTATICA = 4;
        let valor = UMA_VARIAVEL_ESTATICA; // copia o valor
        println!("Constante: {}", TIPO_DE_DADOS);
        println!("Estática: {}", valor);
    }
    imprime();
}

fn imprime() {
    unsafe {
        UMA_VARIAVEL_ESTATICA = 5;
        let valor = UMA_VARIAVEL_ESTATICA; // copia o valor
        println!("Constante: {}", TIPO_DE_DADOS);
        println!("Estática: {}", valor);
    }
}
