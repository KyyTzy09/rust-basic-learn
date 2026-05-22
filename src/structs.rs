#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
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

impl User {
    pub fn update_email(&mut self, new_email: String) -> User {
        self.email = new_email;
        User {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            is_active: self.is_active,
        }
    }
}