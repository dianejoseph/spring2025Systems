// Assignment 3

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main(){
    let secret = 42;
    let mut guess = 30;
    let mut attempts = 0;


    loop {
        attempts += 1;
        match check_guess(guess, secret){
            0=> {
                println!("{} is correct! You guess it in {} attempts!", guess, attempts);
                break;
            }
            1 => println!("{} is too high!", guess),
            -1 => println!("{} is too low!", guess),
            _ => {}
        }
        guess += 1;
    }
}