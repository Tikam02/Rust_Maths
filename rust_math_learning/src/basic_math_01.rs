// Basic mathematical functions
fn add(a: f64,b:f64)-> f64{
    a+b
}

fn multiply(a:f64, b:f64) -> f64 {
    a*b
}

fn power(base: f64, exponent:f64) -> f64 {
    base.powf(exponent)
}

fn absolute_value(x: f64) -> f64{
    if x < 0.0 {
        -x
    } else {
        x
    }

}

// Greatest common divisor using Euclidean Algorithm
fn gcd(mut a:u64, mut b : u64) -> u64 {
    while b != 0{
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}



// Factorial functions (recursive)
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    }else {
        n * factorial(n-1)
    }

}

// Factorial function (iterative)
fn factorial_iterative(n: u64) -> u64 {
    (1..=n).product()
}

fn main() {
    println!("add(5.0,3.0) = {}", add(5.0,3.0));
    println!("multiply(4.0,7.0) = {}", multiply(4.0,7.0));
    println!("power(2.0,8.0) = {}", power(2.0,8.0));
    println!("absolute_value(-15.5) = {}",absolute_value(-15.5));
    println!("gcd(48,18) = {}", gcd(48,18));
    println!("factorial(5) = {}", factorial(5));
    println!("factorial_iterative(5) = {}",factorial_iterative(5));
}  

