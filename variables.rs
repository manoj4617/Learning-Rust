fn main(){
    // let mut x = 2;
    // println!("The numbers is {x}"); 

    // x = 10;
    // println!("The numbers is {x}"); 

    //shadowing
    let m = 10;
    let m = 10 * 2;
    {
        let m = 10 * 3;
        println!("Value of m in inner scope is {m}");
    }
    println!("Value of m in outer scope is {m}");
}