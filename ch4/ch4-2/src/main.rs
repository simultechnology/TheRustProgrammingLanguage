fn main() {
    println!("4-2 starts!");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}', is {},", s1, len);

    let mut s2 = String::from(s1);
    change(&mut s2);
    println!("changed text is '{}'", s2);

    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
