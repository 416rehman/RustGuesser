use rand::Rng;

/** This is the main func **/
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();

    loop {
        guess.clear();  // Clear the string to avoid the previous input
        std::io::stdin().read_line(&mut guess).expect("Failed to read line"); // Read the input

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { //Catches all errors
                println!("Please input a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("You guessed LOWER than the secret number!"),
            std::cmp::Ordering::Greater => println!("You guessed HIGHER than the secret number!"),
            std::cmp::Ordering::Equal => {
                println!("Your guess is correct!");
                break;
            },
        }
    }
}