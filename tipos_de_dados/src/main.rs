fn main() {
    let x:i8 = 99;
    let x:i16 = 9999;
    let x:i32 = 9999999999;
    let x:i64 = 9999999999999999999;
    printl!("{}", x);

    // "u8" -> aceita apenas numeros positivos "i8" -> aceita numeros negativos tbm
}

fn main() {
    let x:f32 = 9.9; // float

    let x:char = 't'; // 1 caractere 

    let x:&str = "ts"; // imutavel msm com o mut
    let mut x = String::from("ts"); // mutavel com o mut 
    x += "tsst"; // ex de mudança 

    printl!("{}", x);
}
