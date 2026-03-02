fn main() {
    let day = 3;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }
}

// Multiple Match

fn main() {
    let day = 6 ;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
}

// Match with range

fn main() {
    let age = 26;

    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        _ => println!("Senior"),
    }
}
