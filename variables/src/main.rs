use std::io;

fn main() {
    //let x:u32 = 5;
    // println!("The value of x is : {x}");

    // x = 6;
    // println!("The value of x is : {x}");

    //Shadowing
    // let x = x + 10;
    // {
    //     let x = x + 20;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x in outer scope is : {x}");

    // let a = [1,2,3,4,5,6];
    // println!("Enter an index");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered is not a number");

    // let ele = a[index];

    // println!("Value at {index} is: {ele}");

   

        let s = String::from("Manoj");
    
        takes_ownership(s);
    
        let x = 32;
        takes_int(x);
        println!("number after sending to fn: {x}");
        //println!("string after sending to fn: {s}");
    
    
    
    fn takes_ownership(str: String){
        println!("string sent is: {str}");
    }
    
    fn takes_int(num: u32){
        println!("number is: {num}");
    }

}
