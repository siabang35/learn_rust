let a = String::from("Hi Danz");
let b = &a;

println!("a = {}", a);
println!("b = {}", b);


// Mutable reference

let mut a = String::from("Hi Danz");
let b = &mut a;

b.push_str("Hi Danz");

println!("{}", b);
