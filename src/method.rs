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

pub trait PaymentMethod {
    fn pay(&self, amount: f64) -> String;
}

pub struct Dana {
    pub account_number: String,
}

impl PaymentMethod for Dana {
    fn pay(&self, amount: f64) -> String {
        format!("Paid {} using Dana account {}", amount, self.account_number)
    }
}