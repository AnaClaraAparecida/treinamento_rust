fn main() {
    let _x: i8 = 99;
    let _x: i16 = 9999;
    let _x: i32 = 999999999;
    let x: i64 = 99999999999999;

    println!("{}", x);
    main1()
}

fn main1() {
    let _x: f32 = 9.9;
    let _x: char = 't';
    let x: &str = "ts"; // sem mut, pois nunca é modificada
    let mut x = String::from(x); // pode reaproveitar o valor
    x += "tsst";

    println!("{}", x);
}