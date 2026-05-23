
#[derive(Debug)]
enum Presiden {
    JOKOWI,
    SBY,
    SOEHARTO,
}

pub  fn arr() {
    let arr : [Presiden; 3] = [Presiden::JOKOWI, Presiden::SBY, Presiden::SOEHARTO];
    for  nama in arr.iter() {
        match nama {
            Presiden::JOKOWI => println!("{:#?} Presiden ke-7 Indonesia", nama),
            Presiden::SBY => println!("{:#?} Presiden ke-6 Indonesia", nama),
            Presiden::SOEHARTO => println!("{:#?} Presiden ke-2 Indonesia", nama),
            _ => println!("Tidak diketahui"),
        }
    }
}