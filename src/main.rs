mod ownership;
mod enums;

fn main() {
    let x = 5;
    let mut y = 10;
    y = 9;

    ownership::ownership();

    let status = enums::Status::Active;
    enums::check_status(status);


    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
