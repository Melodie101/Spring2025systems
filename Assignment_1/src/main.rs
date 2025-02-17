// Assignment 1: Temperature Converter
const FREEZING_POINT: f64 = 32.0;
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_POINT) * 5.0 / 9.0 
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0 / 5.0) + FREEZING_POINT
}

fn main(){
    let mut x = 54.0;
    println!("Temperature in Celsius: {}", fahrenheit_to_celsius(x));
    
    let mut counter = 5;
    while counter != 0 {
            x += 1.0;
            println!("Temperture in Celsius is: {}", fahrenheit_to_celsius(x));
            counter -= 1;
        }
    }