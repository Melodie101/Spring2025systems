
fn sum_of_two(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn main() {

    println!("Sum of two numbers: {}", sum_of_two(5, 4) );

}
   
fn main() {
    let x = 3;
    let y = {

    }
/}


if rem == 0 {
    println!("Divisible");

} else if rem == 1 {
    println!("Remainder = {}", rem);
}

// True/False
let x = true;
match x {
    true => println!("True"),
    false => println!("False"),
}


//Remainder
let x = 5;
let rem = x%3;

let phrase = match rem {
    0 => "Remainder is Zero",
    1 => "Remainder is One",
    2 => {
        println!("This was an amazing choice");
        
    _ => "#",
};

// println!("{}",phrase);


// if statement
fn main() {
    let number = 5;
    
    let divisible_by_two = if number % 2 == 0 {
        "even" // no semicolon, because then it becomes an expression
    } else {
        "odd" // both arms or branches should evaluate to the same type
    };
    
    println!("Number {} is a type of {}", number, divisible_by_two);
}

// Loops
fn main() {
    let mut num = 0;
    loop {
        println!("{}", num);
        num += 1;

        if num == 10 {
            break;
        }
    }
}

fn main(){
    for idx in 0..10 {
        nums.push(idx);

    }
    println!("{}",nums);

}









// Assignment 1
const FREEZING_POINT: u32 = 32;
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - 32.0) * 5.0 / 9.0 
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0 / 5.0) + 32.0
}

fn main(){
    let mut x = 54;
    println!("Temperature in Celsius: {}", fahrenheit_to_celsius(x));

    

}