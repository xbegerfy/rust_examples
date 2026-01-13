use std::io;

fn main() {
    let range_end: u16 = 100;
    println!("Welcome to guess number game, min number is 0, max number is {range_end}");

    let secret_num = rand::random_range(0..range_end);
    loop {
        println!("Please input your guess number");
        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("failed to read user input line");
        let guess_number = guess_number.trim().parse::<u16>();
        let guess_number = match guess_number {
            Ok(number) => number,
            Err(err) => {
                println!(
                    "input error format number, error = {}, please re-enter guess number",
                    err
                );
                continue;
            }
        };

        match guess_number.cmp(&secret_num) {
            std::cmp::Ordering::Less => println!("too small"),
            std::cmp::Ordering::Equal => {
                println!("you win !");
                break;
            }
            std::cmp::Ordering::Greater => println!("too big"),
        }
    }
}
