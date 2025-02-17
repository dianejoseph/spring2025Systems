// Assignment 2

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main(){
    let numbers = [12, 5, 18, 7, 30, 15, 22, 9, 25, 41];

    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            let even_or_odd = if is_even(num) { "Even" } else { "Odd" };
            println!("{}: {}", num, even_or_odd);

        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len(){
        sum += numbers[i];
        i+= 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut max = numbers[0];
    for &num in &numbers {
        if num > max {
            max = num;
        }
    }
    println!("Largest number: {}", max);
}