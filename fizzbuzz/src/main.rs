use std::io;
use std::process;

fn main() {
    let mut input_n = String::new();
    println!("Enter a number");
    io::stdin()
        .read_line(&mut input_n)
        .expect("Give a number");

    let input_n : usize = match input_n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("bro, a number");
            process::exit(1);
        }
    };    

    let answer : Vec<String> = fizzbuzz(input_n);
    
    println!("{:?}", answer);
}

fn fizzbuzz(n : usize) -> Vec<String> {

    let mut vec : Vec<String> = Vec::with_capacity(n);
    for i in 1..n+1 {
        if i % 3 == 0 && i % 5 == 0 {
            vec.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            vec.push("Fizz".to_string());
        } else if i % 5 == 0 {
            vec.push("Buzz".to_string());
        } else {
            vec.push("".to_string());
        }
    }

    return vec;
}
