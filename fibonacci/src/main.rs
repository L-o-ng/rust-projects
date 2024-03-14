use std::io::{stdin, stdout, Write};

fn main() {
    let mut x: u128 = 0;
    let mut y: u128 = 1;
    let mut bound: String = String::new();

    print!("Enter a number: ");
    stdout().flush()
            .expect("msg");
    
    stdin().read_line(&mut bound)
           .expect("msg");

    let bound: i64 = bound.trim().parse().expect("msg");
    

    println!("{x}");
    println!("{y}");
    for number in 2..bound {
        if number % 2 == 1 {
            y += x;
            println!("{y}");
        }
        else {
            x += y;
            println!("{x}");
        }
    }
}
