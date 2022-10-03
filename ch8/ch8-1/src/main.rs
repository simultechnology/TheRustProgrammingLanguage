fn main() {
    println!("ch8-1 starts!");

    let v0: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3];

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("{:?}", v2);

    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);
    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let third= v3[2];
    println!("The third element is {}", third);

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("does_not_exist: {:?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    let v = Vec::from(v);
    let first = &v[0];
    println!("The first element is: {}", first);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // println!("{}", i);
        *i += 50;
    }
    println!("{:?}", &v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", &row);

}
