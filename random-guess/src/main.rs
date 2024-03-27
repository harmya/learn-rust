use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input_number = String::new();
        println!("Input your guess in range 1 to 100");
        
        io::stdin()
            .read_line(&mut input_number)
            .expect("perchance....no line?");
        
        let input_number : u32 = match input_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("perchance....you want to input a number?");
                continue;
            },
        };

        match input_number.cmp(&random_number) {
            Ordering::Less => println!("perchance...you want to guess a higher number?"),
            Ordering::Greater => println!("perchance....you want to guess a lower number?"),
            Ordering::Equal => {
                println!("get rekt");
                break;
            }
        }

    }
}
