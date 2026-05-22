
pub struct Animal {
    pub name: String,
    pub species: String,
    pub age: u8,
}

impl Animal {
    pub fn new(name: String, species: String, age: u8) -> Self {
        Self { name, species, age }
    }

    pub fn make_sound(&self) -> String {
        match self.species.as_str() {
            "Dog" => "Woof!".to_string(),
            "Cat" => "Meow!".to_string(),
            "Cow" => "Moo!".to_string(),
            _ => "Unknown sound".to_string(),
        }
    }
}