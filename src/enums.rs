
pub enum Status {
    Active,
    Inactive,
    Pending,
}

pub fn check_status(status: Status) {
    match status {
        Status::Active => println!("Status is Active"),
        Status::Inactive => println!("Status is Inactive"),
        Status::Pending => println!("Status is Pending"),
    }
}