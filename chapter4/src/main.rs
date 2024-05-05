fn main() {
    println!("Chapter4");
}


// Onwership
// Rust has ownership system that checks at compile time for rules about memory safety.
// Rust also has grabage collector

// Data on the stack
// LIFO
// Fixed size


// Data on the heap
// Heap is less organized
// Heap is slower than stack


// When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) 
//and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.


// Ownership Rules
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.


let hello = "hello"; // hello is the owner of the string "hello"

{
    let hello = "hello"; // hello is the owner of the string "hello"

    // do stuff with hello
} // hello goes out of scope, and the value will be dropped

// we use data-type String to store data on the heap
let s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`


{
    let s1 = String::from("hello");

    //do things with S

} //Rust called drop function to free the memory



// when we assign s1 to s2 this means we make a copy of length, capacity and pointer of s1 to s2 but not the data
{
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // This will throw an error
    // to avoid double free error, Rust does not allow this
    // we don't want tpp free up memory twice
    // Rust considers s1 to be invalid and does not let us use it
} 


// when we want to a copy of the data on the heap we use clone
{
    let s1 = String::from("hello");
    // this will create a deep copy of the data on the heap
    // the copy data can edited without affecting the original data
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}



// Ownership and Functions
// Passing a variable to a function will move or copy the data
// Passing data as a value for specfic data types such as ints, float, bools are easy to make a copy and drop once they go out of scope
// Key words passin by value are Copy and Clone instead of passing by refrence.


let x = 5
let y = x;




// Refrences and Borrowing
// it is used when you want to borrow a value without taking ownership of it
// when the refrence goes out of scope, the value will not be dropped

// however refrences cannot change the value of the data cause it's immutable by default

fn mn(){
    let s1 = String::from("hello");

    // we pass the refrence of s1 to the function
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

}


fn calculate_length(s: &String) -> usize {
    s.len()
}



// if we want to make a mutable refrence we use &mut
fn mn() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// Slice Type
// A slice is a reference contigous sequence of elements in a collection without taking ownership of the data

// String slice
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}