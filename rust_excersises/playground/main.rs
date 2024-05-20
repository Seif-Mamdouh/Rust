use std::io;


fn main() {
    println!("Add or Subtract Numbers");

    let mut counter = 0;

    loop {
        println!("Enter '+' to increment the counter or '-' to decrement the counter: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "q" {
            println!("Quitting program");
            break;
        }

        println!("Enter the number you want to change the counter by: ");
        let mut changeNum = String::new();
        io::stdin().read_line(&mut changeNum).expect("Failed to read line");
        let changeNum : i32 = match changeNum.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse change");
                continue;
            }
        };

        fn add(number: i32, change: i32) -> i32 {
            number + change
        }

        fn subtract(number: i32, change: i32) -> i32 {
            number - change
        }

        match input.trim() {
            "+" => {
                counter = add(counter, changeNum);
                println!("You have chosen to increment the counter");
            }
            "-" => {
                counter = subtract(counter, changeNum);
                println!("You have chosen to decrement the counter");
            }
            _ => {
                println!("Invalid input! Please enter '+' or '-'");
            }
        }

        println!("The counter is {}", counter);
    }
}


use std::io;

fn main (){

    println!("Add two numbers together");


    println!("Enter first number: ");
    let mut n1 = String::new();
    io::stdin().read_line(&mut n1).expect("Failed to read line");
    let n1 : i32 = n1.trim().parse().expect("Please type a number!");

    println!("the first number is {}", n1);


    println!("Enter second number: ");

    let mut n2 = String::new();
    io::stdin().read_line(&mut n2).expect("Failed to read line");
    let n2 : i32 = n2.trim().parse().expect("Please type a number!");

    println!("the second number is {}", n2);


    let sum = n1 + n2;


    println!("The sum of {} and {} is {}", n1, n2, sum);

}