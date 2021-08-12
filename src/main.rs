use std::io;

fn main() {

    let mut guess = String::new();


    // get guess input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    
    let guess = guess.trim();
    
    println!("is the string a palindrome?: {}", is_palindrome(&guess));
}       

fn is_palindrome(guess: &str) -> bool {
    for index in 0..(guess.len() / 2){
        //println!("{}", index);
        if &guess[index..index + 1] == &guess[guess.len() - (1 + index)..guess.len() - index] {
            continue;
        }
        else {
            return false
        }
    }
    true 
}