use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value,simulated_random_number);

    let mut c1 = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("-----------------------------");
    println!("c1 usa 2: {}",c1.value(2));
    println!("c1 usa 3: {}",c1.value(3));
    println!("c1 usa 3: {}",c1.value(3));
    println!("c1 usa 3: {}",c1.value(3));

    let x = vec![1,2,3];

    let equal_to_x = move |z| z == x;

    let y = vec![1,2,3];

    assert!(equal_to_x(y));
}

//fn simulated_expensive_calculation(intensity: u32) -> u32{
//  println!("calculating slowly...");
//  thread::sleep(Duration::from_secs(2));
//  intensity
//}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32,u32>
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher { calculation , values: HashMap::new() }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg){
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut expensive_closure = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Today, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }else{
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}
