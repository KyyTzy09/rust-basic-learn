
pub fn vec_example() {
    let mut numbers: Vec<String> = Vec::new();
    numbers.push(String::from("one"));
    numbers.push(String::from("two"));
    numbers.push(String::from("three"));

    println!("Numbers: {:?}", numbers);
}