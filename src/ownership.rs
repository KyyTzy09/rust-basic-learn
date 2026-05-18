
pub fn ownership() {
    let s1 = String::from("Hello");
    let s2 = s1;

    // Ini akan error karena S1 sudah dipindahkan ke S2, jadi S1 tidak lagi valid
    // println!("s1: {}", s1); 

    println!("s2: {}", s2);

    // Borrowing
    let s3 = String::from("World");
    let length = hitung_length(&s3);
    println!("The length of '{}' is {}.", s3, length);
}

fn hitung_length(s : &String) -> usize {
    s.len()
}