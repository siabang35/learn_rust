let score = 90;

if score >= 90 {
    let result = "Pass";
    println!("Result: {}", result);
}

println!("Result: {}", result);


// Nested scope

let x = 10;

if x > 5 {
    let y = 20;
    println!("x: {}, y: {}", x, y);
}

println!("x: {}, y: {}", x, y);


// Shadowing

let x = 10;
let x = x + 7;
let x = x * 3;

println!{"x: {}", x};


// Shadowing with different types

let x = 10;
let x = "Hello";

println!{"x: {}", x};
