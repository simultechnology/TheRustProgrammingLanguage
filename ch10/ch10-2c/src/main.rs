use std::fmt::Display;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("ch10-2c starts!!");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let pair = Pair {
        x: "ishi",
        y: "taka",
    };
    let pair2 = Pair::new("ishi2", "taka2");

    println!("{:?}", pair);
    println!("{:?}", pair2);

    pair2.cmp_display();

    let pair3 = Pair::new(5, 7);
    pair3.cmp_display();


    let number_list2 = vec![1000, 30];

    // this is a compile error duw to unsatisfied trait bounds
    // let pair4 = Pair::new(number_list, number_list2);
    // pair4.cmp_display();
}
