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


