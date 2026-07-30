fn main() {
    let x:i8 = 99;
    let x:i16 = 9999;
    let x:i32 = 999999999;
    let x:i64 = 99999999999999;

    println!("{}", x);
    main1()

    // "u8" -> aceita apenas numeros positivos "i8" -> aceita numeros negativos tbm
}

fn main1() {
    let x:f32 = 9.9; // float

    let x:char = 't'; // 1 caractere 

    let mut x:&str = "ts"; 
    let mut x = String::from("ts"); // mutavel com o mut 
    x += "tsst"; // ex de mudança 

    println!("{}", x);
}