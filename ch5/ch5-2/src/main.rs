#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("ch5-2 starts!!");

    let width1 = 30;
    let height1 = 50;

    let rectangle = Rectangle {
        width: width1,
        height: height1,
    };

    //let rect1 = (width1, height1);

    println!(
        "The are of the rectangle is {} square pixels.",
        area(&rectangle)
    );
    println!("{:?}", rectangle);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
