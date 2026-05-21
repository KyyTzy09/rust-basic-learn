#[derive(Debug)]
pub struct User {
    id: u32,
    username: String,
    email: String,
    password: String,
    is_active: bool,
}

pub fn fill_struct(
    id: u32,
    username: String,
    email: String,
    password: String,
    is_active: bool,
) -> User {
    User {
        id,
        username,
        email,
        password,
        is_active,
    }
}
