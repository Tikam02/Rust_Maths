fn main() {
    // Basic arithmetic operations
    let a = 10.0_f64;
    let b = 3.0_f64;
    
    println!("Addition: {} + {} = {}", a, b, a + b);
    println!("Subtraction: {} - {} = {}", a, b, a - b);
    println!("Multiplication: {} * {} = {}", a, b, a * b);
    println!("Division: {} / {} = {}", a, b, a / b);
    println!("Exponentiation: {}^{} = {}", a, b, a.powf(b));
    println!("Square root of {}: {}", a, a.sqrt());
    
    // Integer operations
    let x = 17i32;
    let y = 5i32;
    println!("Integer division: {} / {} = {}", x, y, x / y);
    println!("Modulo: {} % {} = {}", x, y, x % y);
}
