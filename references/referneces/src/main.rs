//Example of references

fn adds_10(num: &u32) -> u32{
    let _ = &(num + 10);

    *num
}

fn append(str: &mut String, str_to_append: &String) {
    str.push_str(str_to_append);
}

fn main() {
    let number: u32 = 20;
    let num: u32 = adds_10(&number);

    println!("Returned value {}", num);

    let mut str: String = String::from("Manoj");
    /*
    * References are immutable and cannot be changed 
    * the only way a function can change a reference 
    * is by receiving it as a mutable one `&mut`
     */
    append(&mut str, &String::from("Kumar"));

    println!("Final String : {}", str);
}
