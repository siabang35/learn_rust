enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Green => println!("Go"),
    }
}

// Enum with data

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Write(String::from("Hi Danz"));

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})"),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})")
    }
}


// Enums with Data

enum LoginStatus {
    LoggedIn,
    LoggedOut,
    Pending,
    Success(String),
    Error(String),
}

fn main() {
    let status = LoginStatus::Success(String::from("Login Success"));

    match status {
        LoginStatus::LoggedIn => println!("Logged In"),
        LoginStatus::LoggedOut => println!("Logged Out"),
        LoginStatus::Pending => println!("Pending"),
        LoginStatus::Success(message) => println!("{}", message),
        LoginStatus::Error(message) => println!("{}", message),
    }
}