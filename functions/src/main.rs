fn another_function(x: u32){
    println!("Value passed is: {x}");
}

fn and_another_function(age: i32, name: &str){
    println!("{name} is {age} years old");
}

fn add(val1: u32, val2: u32) -> u32 {
    val1 + val2
}

fn main() {
    println!("This is main function");
    //another_function(23);
    //and_another_function(23, /*String::from("Manoj")*/ "manoj");
    // let y = {
    //     let x = 3;
    //     x + 1;
    // };
    // println!("Value of y: {y}");

    let ret = add(3,4);
    println!("the sum of 3 and 4 = {ret}");
}
