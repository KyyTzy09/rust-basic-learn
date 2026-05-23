pub fn if_conditional() {
    let age: i32 = 20;

    if age < 10 {
        println!("child");
    } else if age < 30 {
        println!("teen");
    } else {
        println!("adult");
    }
}

pub fn match_conditional() {
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
