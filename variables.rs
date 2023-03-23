fn main(){

    let s = String::from("Manoj");

    takes_ownership(s);

    let x = 32;

    println!("number after sending to fn: {x}");
    println!("string after sending to fn: {s}");

}

fn takes_ownership(str: String){
    println!("string sent is: {str}");
}

fn takes_int(num: u32){
    println!("number is: {num}");
}