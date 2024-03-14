use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;


fn main() {
    
    let secret: u32 = get_rand();
    
    println!("Guess the Number!tm\n");

    loop {
        let mut guess: String = String::new();
        print!("Enter guess: ");

        io::stdout()
            .flush()
            .expect("Error!");


        io::stdin()
            .read_line(&mut guess)
            .expect("Error!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small")
        }
    }
    

}

fn get_rand() -> u32{
    let nanos: u32 = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("msg")
        .subsec_micros();
    
    1 + nanos % 100
}