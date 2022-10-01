use std::io;    //Bringing input/output lib to scope
                // io lib comes from standard lib std

fn main() {
    println!("Guess a number !!");

    println!("Please input your guess");

    let mut guess = String::new(); 
    //by default vaiables in rust are immutable 
    //by adding mut keyword before the variable name we make it mutable
    // = operator will bind the value to the variable name
    //::new() is a associated function which is implemented on a type 
    //it creates a new instance of the type specified, in this case string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //takes user input and stores it in guess 
    //&mut makes it mutable so that stdin() can take input and append it to the string
    //& creates a reference type which makes it easily accessible to all the 
    //parts in the code without copying the data in multiple places in the memory
    println!("You guessed : {guess}");
}
