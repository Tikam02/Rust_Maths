//simple exclusive range
fn main(){
for i in 1..5{
    println!("Exclusive Range = {}",i) // 1,2,3,4
    }
    println!();

//Inclusive range
for j in 1..=5 { 
    println!("Inclusive Range = {}",j)
    }
    println!();


// With Characters
for c in 'a'..='d'{
    println!("{}",c)
}
println!();


//Creating Vector from a Range
let vec: Vec<i32> = (1..5).collect(); //vec = [1,2,3,4]
let inclusive_vec: Vec<i32> = (1..=5).collect(); // vec = [1,2,3,4,5]

print!("vec:");
for k in &vec {
    print!("{}",k);
}
println!();

print!("inclusive_vec:");
for l in &inclusive_vec {
    print!("{}",l);
}
println!();

}
