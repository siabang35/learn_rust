let greeting: &str = "Hello, Rust!";
println!("{}", greeting);


// Use Push str

let mut message = String::from("Hi");
message.push_str("Danz");

println!("{}", message);


// Concatenate strings

let s1 = String::from("Hi");
let s2 = String::from("Danz");
let result = s1 + &s2;

println!("{}", result);


// Format! macro

let s1 = String::from("Hi");
let s2 = String::from("Danz");
let result = format!("{} {}", s1, s2);

println!("{}", result);


// String indexing

let s = String::from("Hi");
let first_char = s[0];

println!("{}", first_char);


// String slicing

let s = String::from("Hi Danz");
let slice = &s[0..2];

println!("{}", slice);

// Strung length

let s = String::from("Hi, Danz");
println!("Length: {}", s.len());

// Check if string contains substring

let s = String::from("Hi, Danz");
println!("Contains 'Danz': {}", s.contains("Danz"));
