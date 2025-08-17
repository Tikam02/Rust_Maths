// Check if a number is prime
fn is_prime(n: u64) -> bool{
    if n<2 {
        return false;
    }
    if n === 2{
        retrun true;
    }
    if n % 2 == 0{
        return false;
    }

    let limit = ( n as f64 ).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0{
            return false;
        }
    }
    true
}


