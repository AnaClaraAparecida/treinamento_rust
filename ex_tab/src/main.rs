fn main() {
    let valor_tab: i32 = 2;
    for mult: i32 in 1..=10{
        println!("{} X {} = {}", mult, valor_tab, (mult * valor_tab));

    }
}

fn main1() {
    let valor_tab: String = String::new();
    io::stdin()
        .read_line(buf: &mut valor_tab) Result<usize, Error>
        .expect(msg: "Falha ao ler a linha");
let valor_tab: i32 = valor_tab.trim().parse().expect(msg:"Por favor")
    for mult: i32 in 1..=10{
        println!("{} X {} = {}", mult, valor_tab, (mult * valor_tab));

    }

}