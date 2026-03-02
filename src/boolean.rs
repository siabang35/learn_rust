let is_programmer: bool = true;
let is_student: bool = false;
let is_adult: bool = true;
let is_employed: bool = true;

println!("Is programmer: {}", is_programmer);
println!("Is student: {}", is_student);

let is_adult: bool = is_student && is_programmer;
println!("Is adult: {}", is_adult);

let is_not_adult: bool = !is_adult;
println!("Is not adult: {}", is_not_adult);

let is employed: bool = is_adult && is_employed;
println!("Is employed: {}", is_employed);


//Boolean for comparison

let age = 26;
let can_vote: bool = age >= 18;
println!("Can vote if the age is 18 or above: {}", can_vote);

//Boolean in if statement

let is_logged_in = true;
if is_logged_in {
    println!("Welcome to the dashboard");
} else {
    if is_admin {
        println!("Welcome to the admin panel");
    } else {
        println!("Please log in to continue");

    }
}