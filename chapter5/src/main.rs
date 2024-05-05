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