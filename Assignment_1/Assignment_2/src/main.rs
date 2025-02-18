// Assignment 2: Number Analyzer

fn is_even(n: i32) -> bool {
      n % 2 == 0 
}

fn main() {
    let nums = [1,2,3,4,5,6,7,8,9,10];

    // For loop iterating through the array.
    for &num in nums.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("Number {} is even?: {}", num, is_even(num));
    }
}
    
    
    // Sum of all numbers using a while loop.
    let mut sum = 0;
    let mut idx = 0;
    while  idx < nums.len() {
        sum += nums[idx];
        idx += 1;
        
    }
    println!("Sum of Numbers: {}",sum);

    // Finding the largest number
    let mut largest = nums[0];
    for &num in nums.iter(){
        if num > largest {
            largest = num;
        }
    }
    println!("Largest Number: {}", largest);
}


