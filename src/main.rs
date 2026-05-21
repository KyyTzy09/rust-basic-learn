mod ownership;
mod enums;
mod structs;

fn main() {
    let x = 5;
    let mut y = 10;
    y = 9;

    ownership::ownership();

    let status = enums::Status::Active;
    enums::check_status(status);


    let user = structs::fill_struct(
        1,
        String::from("john_doe"),
        String::from("YHs1m@example.com"),
        String::from("password123"),
        true,
    );

    println!("user: {:#?}\n", user);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

