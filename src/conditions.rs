if 10 > 5 {
    println!("10 is greater than 5");
} else {
    println!(10 is not greater than 5);
}

let age = 26;

if age >= 18 {
    println!("You are an adult");
} else {
    println!("You are a minor");
}

// if expression

let time = 20;
let greeting = if time < 12 {
    "Good Morning."
} else {
    "Good Afternoon."
};

println!("{}", greeting);

// if else if else

let score = 85;

if score >= 90 {
    println!{"Grade A"};
} else if score >= 80 {
    println!("Grade B");
} else if score >= 70 {
    println!("Grade C");
} else if score >= 60 {
    println!("Grade D");
} else {
    println!("Grade F");
}

// Simplified Syntax 
let time = 20;
let greeting = if time < 18 { "Good Day." } else { "Good Evening. "};

println!("{}", greeting);
