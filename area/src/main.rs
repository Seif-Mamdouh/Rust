fn main () {
    let l = 4;
    let w = 5;

    println!("The area of the rectangle is {} square pixels.", 
    area(l, w));
}


fn area(length: u32, width: u32) -> u32 {
    length * width
}


// Refactoring with Structs

struct Rec {
    length: u32,
    width: u32,
}


fn main() {
    let rect1 = Rec { 
        length: 5, 
        width: 4,
     };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}


fn area(rectangle: &Rec) -> u32 {
    rectangle.length * rectangle.width
}