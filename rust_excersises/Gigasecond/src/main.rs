use time::{PrimitiveDateTime, macros::format_description};

fn date(s: &str) -> String {
    let dt = time::parse(s, &format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")).unwrap().to_string();
}

fn main() {

    println!("{}", date("2024-04-23 06:12:23"))
}



// use time::{PrimitiveDateTime, macros::format_description};

// fn gigaSecond(s: &str) -> String {
//     let dt = PrimitiveDateTime::parse(s, &format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")).unwrap();
//     let gigasecond_later = dt + time::Duration::seconds(1_000_000_000);
//     gigasecond_later.format(&format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")).unwrap().to_string()
// }