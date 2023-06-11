fn first_word(s: &String) -> &str{
    let word_bytes = s.as_bytes();

    for (i, &item) in word_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main(){

    let _str: String = String::from("Hello Manoj!");
    let ret_str = first_word(&_str);

    println!("First word is : {}", ret_str);

}