let person = ("Danz", 26, "Male");

println!{"Name: {}", person.0};
println!{"Age: {}", person.1};
println!{"Gender: {}", person.2};

// Destructuring

let (name, age, gender) = person;

println!{"Name: {}", name};
println!{"Age: {}", age};
println!{"Gender: {}", gender};


