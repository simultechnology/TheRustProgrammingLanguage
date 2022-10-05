use std::collections::HashMap;

fn main() {
    println!("ch8-3 start!!");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let is = initial_scores.iter();

    let scores2: HashMap<_, _> = teams.iter().zip(is).collect();
    println!("{:?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
   // println!("{}", field_name);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);
    match score {
        None => (),
        Some(x) => println!("{:?}", x)
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("{:?}", scores);
    println!("--------update---------");

    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let numbers = vec![8, 4, 90, 4, 7, 19, 2, 9, 123];
    let value = get_mean(&numbers);
    println!("{:?}", numbers);
    println!("mean value: {:.5}",  value);
}

fn get_mean(numbers: &Vec<i64>) -> f64 {
    let count = numbers.len();
    let total: i64 = numbers.iter().sum();
    let value: f64 = (total as f64 / count as f64);
    value
}
