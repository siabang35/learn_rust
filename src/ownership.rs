let a = String::from("Hi Danz");
let b = a;

println!("{}", a);
println!("{}", b);


// Clone

let a = String::from("Hi Danz");
let b = a.clone();

println!("{}", a);
println!("{}", b);

// Move

fn main() {
    let s1 = String::from("Hi Danz");
    let s2 = s1;


    println!("{}", s1);
    println!("{}", s2);
}

// Function & Ownership

fn main() {
    let s = String::from("Hi Danz");
    takes_ownership(s);
    println!("{}", s);

} fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn main() {
    let x = 5;
    makes_ownership(x);
    println!("{}", x);
}
fn makes_ownership(some_integer: i32){
    println!("{}", some_integer);
}
