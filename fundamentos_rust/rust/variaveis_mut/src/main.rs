fn main() {
    let x: i32 = 5; // -> imutavel
    let mut y: i32 = 5; // -> mutavel
    y += 6;
    println!("Hello, world! - {} and - {}", y, x);
}