//CRATE 
//crate is rust libaries 
use std::io; //libary for reading and writing 
use std::cmp::Ordering; //a libary that has ordering enum?
use rand::Rng; //libary for generating random numbers

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    println!("The Secret number is: {secret_number}");
loop {

    println!("Please input your guess.");

    //creates a variables new guess
    //new() is associated function most type in rust has them 
    //it basically creates an empty string in the memory ready to be used 
    let mut guess: String = String::new();

    //import the standard libary 
    //tools created by rust
    //this creates the user input field in the terminal
    io::stdin()

        //This line read the input from the temrinal and binds it to guess
        //&mut guess is shared referneces meaining its not creating a new value of anything
        //it is referencing the value without copying 
        //so its basically just pointing at it saying "change this"
        //only one &mut guess (changable) can exist at a time but multiple &guess (non-changeable) can exist
        .read_line(&mut guess) 

        //readline also returns an result value(enum) with is sucess type/variant and error type/variant
        //in case of failure/error type it will return this
        .expect("Failed to read line"); //will cause the program to crash

    //convert guess user input to a interger/number
    //this is called shadowing basically resuing a value 
    //idk if its copying or just pointing/borrowing
    //trim method remove whitespace cartain OS adds space when doing stuff like clicking enter
    //parse converts to another type
    //by using :i32 parse knows what to convert it too
    //parse return a result type so I'm using expect to handle the error
    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number!!!");
            continue
        },
    };

    
    //this is match pattern use to compare values and then give an outcome 
    //if the value match 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break}
        }
        //this is self explantory
    println!("You guessed: {guess}")
};

}
