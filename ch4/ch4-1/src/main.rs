fn main() {

    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}", s);

    let s = String::from("hello2");
    let s = takes_ownership2(s);

    println!("{}", s);

    // sample of giving ownership
    let s3= gives_ownership();
    let s4 = takes_ownership2(s3);

    //println!("----------\n{}", s3); // value already moved and error happens
    println!("----------\n{}", s4);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let s = String::from("hello3");
    s
}

fn takes_ownership2(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
