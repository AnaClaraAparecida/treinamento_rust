use std::io;

fn main() {
    tabuada_fixa();
    tabuada_com_input();
}

fn tabuada_fixa() {
    let valor_tab: i32 = 2;
    for mult in 1..=10 {
        println!("{} X {} = {}", mult, valor_tab, mult * valor_tab);
    }
}

fn tabuada_com_input() {
    let mut valor_tab = String::new();
    io::stdin()
        .read_line(&mut valor_tab)
        .expect("Falha ao ler a linha");

    let valor_tab: i32 = valor_tab.trim().parse().expect("Por favor, digite um número válido");

    for mult in 1..=10 {
        println!("{} X {} = {}", mult, valor_tab, mult * valor_tab);
    }
}