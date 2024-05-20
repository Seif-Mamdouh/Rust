fn reverse_string(s: &str) -> String {
    //expresssion
    s.chars().rev().collect()
}

fn main() {
    let name = String::from("Seif");
    let mut last_name = String::from("Mamdouh");

    let reversed_name = reverse_string(&name);

    let last_name: String = String::from("Mohamed")
    let full_name = reversed_name + " " + &last_name;

    println!("The full name is {}", full_name);
}




// lessons learned
// String literals are immutable and they stored in a binary file
// to make string mutable you have to use let mut var = String::from("string");

//expressions evaluate the value but doesn't return anything also they dont't end in semicolon