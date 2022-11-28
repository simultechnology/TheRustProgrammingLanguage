use std::collections::HashMap;
use std::iter::Map;
use std::thread;
use std::time::Duration;

fn main() {
    println!("ch13-1 starts!");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // println!("{}", simulated_expensive_calculation(100));

}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result =
        // simulated_expensive_calculation(intensity);
        Casher::new( |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
            // expensive_closure(intensity)
        );

        println!(
            "Today, do {} situps!",
            expensive_result.value(intensity)
            // expensive_closure(intensity)
        );

    } else {
        if random_number ==3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
                // expensive_closure(intensity)
            )
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Casher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Casher<T>
    where T: Fn(u32) -> u32
{
    
    fn new(calculation: T) -> Casher<T> {
        Casher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(
                    arg,
                    v,
                );
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Casher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
