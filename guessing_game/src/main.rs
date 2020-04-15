use std::io; //Note => :: means 'Associated function'
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0, 11); 
    //println!("The secret number is: {}\n", secret_number);

    loop {
        println!("\nPlease input your guess, (number please): ");
        let mut guess = String::new(); //Mutable
        //Handling Potential Failure, don´t forget this
        io::stdin().read_line(&mut guess).expect("Failed to read line"); 

        //Shadow variable and 32-bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You number: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            }
        }
    }
}