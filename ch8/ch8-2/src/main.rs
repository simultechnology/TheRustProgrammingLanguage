fn main() {
    println!("ch8-2 starts!!");

    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    //println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = "1高い山だよ";
    let s = &hello[0..4];
    println!("{}", s);

    let s = &hello[0..1];
    println!("{}", s);
    println!("------------------");

    for c in hello.chars() {
        println!("{}", c);
    }
    println!("------------------");
    for b in hello.bytes() {
        println!("{}", b);
    }
}
