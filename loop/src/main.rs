fn main() {
    loop {
        println!("Hello, world!");
        break;
    }
  
}

fn main1 () {
    let mut x: i32 = 1;
    while x < 20 {
        println!("Hello, world! {}", x);

        if x > 10 { break; }

        x += 1;
    }
}

fn main2 () {
    let mut x: i32 = 1;
    while x <= 20 {

        if (x == 10) || (x == 5) {
            x += 1;
            continue;
        }
        println!("Hello, world! {}", x);

        //if x > 10 { break; }

        x += 1;
    }
}

fn main3 () {
    for number: i32 in 1..=4 {
        println!("Numero: {}", number);
    }
}