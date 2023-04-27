use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(0..10);

    loop {
        println!("Please enter a guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(
            "Failed to read line"
        );

        let guess = guess.trim().parse::<i32>().expect("Please enter a number");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You are guess!"); 
                break;
            },
            Ordering::Greater => println!("Your number is bigger"),
            Ordering::Less => println!("Your number is less"),
        }

        println!("___________________")
    }
}
