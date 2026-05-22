mod ownership;
mod enums;
mod structs;
mod method;

fn main() {
    let x = 5;
    let mut y = 10;
    y = 9;

    ownership::ownership();

    let status = enums::Status::Active;
    enums::check_status(status);
    
    let user = structs::User::update_email(&mut structs::User{id: 1, username: String::from("john_doe"), email: String::from("YHs1m@example.com"), password: String::from("password123"), is_active: true}, String::from("halo"));

    println!("user: {:#?}\n", user);

    // let user = structs::fill_struct(
    //     1,
    //     String::from("john_doe"),
    //     String::from("YHs1m@example.com"),
    //     String::from("password123"),
    //     true,
    // );

    // let mut user2 = structs::fill_struct(
    //     2,
    //     String::from("karim_doe"),
    //     String::from("ahmadiah@example.com"),
    //     String::from("password123"),
    //     true,
    // );

    // user2 = structs::User{
    //     id: 2,
    //     username: String::from("karim_doe"),
    //     ..user
    // };

    // println!("user: {:#?}\n", user2);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let animal = method::Animal::new(String::from("Buddy"), String::from("Dog"), 3);
    let sound = animal.make_sound();
    println!("{} the {} says: {}", animal.name, animal.species, sound);
}

