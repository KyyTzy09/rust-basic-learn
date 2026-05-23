use std::collections::HashMap;


pub fn hash_map() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!( "Scores: {:?}", scores);
}