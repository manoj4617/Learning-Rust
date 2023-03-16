use std::io;    //Bringing input/output lib to scope
                // io lib comes from standard lib std
use rand::Rng;   
use std::cmp::Ordering; 

fn main() {
    println!("Guess a number !!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("Please input your guess");

        let mut guess = String::new(); 
        //by default variables in rust are immutable 
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
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        //the above code shadows the existing guess variable becuase the existing guess variable
        //is of type string and need to converted to number
        //trim() removes spaces before and after the input and parse() converts the input to intended type
        //in this case its u32 which is number of 32 bits
        //parse returns Result enum which has two variants Ok and Err
        

        // println!("You guessed : {guess}");

        // if guess < secret_number {
        //     println!("Too small!!");
        // }
        // else if guess > secret_number {
        //     println!("Too biggg!!");
        // }
        // else if guess == secret_number {
        //     println!("YOU WINNN!!");
        //     break;
        // }

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal =>{ 
                println!("YOU WIN!!");
                break;
            },
        }
    }
}
