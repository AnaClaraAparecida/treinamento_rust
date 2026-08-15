fn main() {
    loop_example();
    while_example();
    while_continue_example();
    for_example();
}

fn loop_example() {
    loop {
        println!("Hello, world!");
        break;
    }
}

fn while_example() {
    let mut x: i32 = 1;
    while x < 20 {
        println!("Hello, world! {}", x);

        if x > 10 {
            break;
        }

        x += 1;
    }
}

fn while_continue_example() {
    let mut x: i32 = 1;
    while x <= 20 {
        if (x == 10) || (x == 5) {
            x += 1;
            continue;
        }
        println!("Hello, world! {}", x);

        x += 1;
    }
}

fn for_example() {
    for number in 1..=4i32 {
        println!("Numero: {}", number);
    }
}