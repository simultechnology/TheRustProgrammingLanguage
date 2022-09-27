use std::env;

fn main() {
    println!("start ch4-3");

    let mut text = String::from("What kind of student");

    let word = first_word(&text);

    let word2 = String::from(word);

    println!("{}", word);

    text.clear();

    // let word2 = String::from(word);
    //
    println!("{}", text);
    println!("{}", word2);

    let my_string = "hello world";
    let my_word = first_word(my_string);
    println!("my word : {}", my_word);

    let text2 = String::from("Bonjour monsieu!");
    let my_word2 = first_word(&text2[..]);
    println!("my word2 : {}", my_word2);

    let x = env::args().count();

    let a = [1, 2, 3, 4, 5];
    let slice =  a[x];
    println!("{:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

