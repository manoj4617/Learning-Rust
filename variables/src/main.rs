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

    let a = [1,2,3,4,5,6];
    println!("Enter an index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered is not a number");

    let ele = a[index];

    println!("Value at {index} is: {ele}");

}
