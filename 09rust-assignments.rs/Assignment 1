// Assignment 1

const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0 
}

fn celcius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main(){
    let mut temp_f = 32.0;

    println!("{:.2} degrees Fahrenheit = {:.2} degrees Celcius", temp_f, fahrenheit_to_celcius(temp_f));

    for i in 1..=5 {
        let new_temp_f = temp_f + i as f64;
        println!("{:.2} degrees Fahrenheit = {:.2} degrees Celcius", new_temp_f, fahrenheit_to_celcius(new_temp_f));
    }
}