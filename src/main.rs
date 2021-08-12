use std::io;

fn main() {

    // create a new empty mutable string
    let mut guess = String::new();


    // get user input 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    
    // trim the string of unwanted characters
    let guess = guess.trim();
    
    println!("is the string a palindrome?: {}", is_palindrome(&guess));
}       

fn is_palindrome(guess: &str) -> bool {
    for index in 0..(guess.len() / 2){
        if &guess[index..index + 1] == &guess[guess.len() - (1 + index)..guess.len() - index] {
            continue;
        }
        else {
            return false
        }
    }
    true 
}

