struct Animal {
    name: String,
    age: i32;
    gender: String;
    color: String;
    is_wild: bool;
}

fn main() {
    let animal = Animmal {
        name: String::from("Tiger");
        age: 4;
        gender: String::from("Male");
        color: String::from("Orange");
        is_wild: true;
    }

    println!{:?}
}

// Struct with method

impl Animal {
    fn new(name: String, age: i32, gender: String, color: String, is_wild: bool) -> Animal {
        Animal {
            name,
            age,
            gender,
            color,
            is_wild,
        }
    }
}