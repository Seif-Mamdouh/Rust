// Data Types
// Scalar Types
// Compound Types



// Scalar Types 
// Integers
    // i8, i16, i32, i64, i128, isize
    //  becareful with integer overflow
// Floating-Point Numbers
// Booleans
// Characters



fn main() {

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation
}



// Compound Types
// Tuples
// Arrays
// can group multiple values into one type


fn main2() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;


    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!(first, second)
}


fn main(){
    let x = 5;

    another_fn(x);
}


fn another_fn(x: i32){
    let x = x + 6;
    println!("The value of x is: {}", x);
}   
'

//statement and expressions
// statements are instructions that perform some action and do not return a value
// expressions evaluate to a resulting value
// Expressions do not include ending semicolons.
fn main() {
    let x = 5; // statement

    let y = { // y is initialized by the block expression
        let x = 3; // statement
        x + 1 // expression
    };

    println!("The value of y is: {}", y); // statement
}


// functions with return values
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}


//control flow
// if expressions

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}


Using if in a let Statement
Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable, as in Listing 3-2.

Filename: src/main.rs

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}



//loops

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}


//while loop
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


//for loop
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{element}");
    }
}