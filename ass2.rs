fn main() {
    let mut num1 = 10;
    let mut num2 = 20;

    // Add two numbers
    let sum = add_numbers(&mut num1, &mut num2);
    println!("The sum of {} and {} is {}", num1, num2, sum);

    // Subtract two numbers
    let difference = subtract_numbers(&mut num1, &mut num2);
    println!("The difference of {} and {} is {}", num1, num2, difference);

    // Multiply two numbers
    let product = multiply_numbers(&mut num1, &mut num2);
    println!("The product of {} and {} is {}", num1, num2, product);

    // Divide two numbers
    let quotient = divide_numbers(&mut num1, &mut num2);
    println!("The quotient of {} and {} is {}", num1, num2, quotient);
}

fn add_numbers(num1: &mut i32, num2: &mut i32) -> i32 {
    *num1 += *num2;
    return *num1;
}

fn subtract_numbers(num1: &mut i32, num2: &mut i32) -> i32 {
    *num1 -= *num2;
    return *num1;
}

fn multiply_numbers(num1: &mut i32, num2: &mut i32) -> i32 {
    *num1 *= *num2;
    return *num1;
}

fn divide_numbers(num1: &mut i32, num2: &mut i32) -> i32 {
    *num1 /= *num2;
    return *num1;
}