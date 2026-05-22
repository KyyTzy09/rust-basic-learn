
#[derive(Debug)]
struct Presiden {
    JOKOWI: String,
    SBY: String,
    SOEHARTO: String,
}

pub  fn arr() {
    let presiden = Presiden {
        JOKOWI: String::from("Joko Widodo"),
        SBY: String::from("Susilo Bambang Yudhoyono"),
        SOEHARTO: String::from("Soeharto"),
    };

    let mut arr : [String; 3] = [presiden.JOKOWI, presiden.SBY, presiden.SOEHARTO];
    for  nama in arr.iter() {
        match nama.as_str() {
            "Joko Widodo" => println!("Presiden ke-7 Indonesia"),
            "Susilo Bambang Yudhoyono" => println!("Presiden ke-6 Indonesia"),
            "Soeharto" => println!("Presiden ke-2 Indonesia"),
            _ => println!("Tidak diketahui"),
        }
    }
}