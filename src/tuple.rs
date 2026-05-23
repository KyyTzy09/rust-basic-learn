
pub fn tuple() {
    let person: (&str, u32, &str) = ("Alice", 30, "Engineer");
    println!("Name: {}, Age: {}, Profession: {}", person.0, person.1, person.2);
}