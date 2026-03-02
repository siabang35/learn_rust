let a = String::from("Hi Danz");
let b = &a;

println!("{}", b);


// Mutable borrowing

let mut a = String::from("Hi Danz");
let b = &mut a;

b.push_str("Hello");

println!("{}", b);


// Multiple borrowing

let mut a = String::from("Hi Danz");
let b = &a;
let c = &a;


println!("{} {}", b, c)

// Mutable and immutable borrowing

let mut a = String::from("Hi Danz");
let b = &a;
let c = &mut a;

println!("{} {}", b, c);

// Dangling pointer fixed

fn main() {
    let r1 = String::from("Hi Danz");
    let r2 = &r1;

    println!("{}", r2);
}

