struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl <T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("ch10-1 starts!");

    let number_list = vec![34, 500, 25, 100, 65];
    println!("{:?}", number_list);

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    println!("---------------------");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p: Point<f32, f32> = Point { x: 10.0, y: 10.0 };
    println!("p.x = {}", p.distance_from_origin());

    println!("---------------------");

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.u = {}", p3.x, p3.y);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest= &list[0];

    for number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}
