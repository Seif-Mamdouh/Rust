fn main() {
    println!("Hello, world!");
}


// a struct is a custom data type that 
//lets you name and package together multiple related values that make up a meaningful group.
// entire instance must be mutable


fn main(){
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }


    // to get a specfic value we use the dot notation
    let user1 = User.username
}


// we can create new instances of structs by using the struct name and then the values for each field in curly brackets

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn mn(){
    let person1 = User {
        email: String::from("someone@gmail.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    }

}


// Tuple structs are useful when the fields of the struct do not require named access, 
// and the order and type of the fields is the primary semantic information.