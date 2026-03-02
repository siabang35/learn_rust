let mut count = 1;

loop {
    println!{"Hi Danz"};

    if count == 10 {
        break;
    }
    count += 1;
}

//While loop

let mut count = 1;

while count <= 10 {
    println!{"Hi Danz"};
    count +=1;
}

// Stop while loop

let mut num = 1;

while num <= 10 {
    if num == 5 {
        break;
    }
    println!("Hi Danz");
    num += 1;
}

//Skip while loop

let mut num = 1;

while num <= 10 {
    if num == 5 {
        num += 1;
        continue;
    }
    println!("Hi Danz");
    num += 1;
}

//For loop

for count in 1..=10 {
    println!{"Hi Danz"};
}

