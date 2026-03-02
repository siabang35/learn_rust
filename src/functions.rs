//Create a function

fn say_hello() {
    println!{ "Hello Danz"};
}

say_hello();

//Function with parameters

fn greet(name: &str){
    println!("Hello {}", name);

}

greet("Danz");


//Function with return value

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

let sum = add(6, 9);
println!("Sum: {}", sum);


//Function with multiple return values

fn get_user() -> (String, i32) {
    return ("Danz".to_string(), 26);
    let (name, age) = get_user();
    println!("Name: {}", name);
    println!("Age: {}", age);
}