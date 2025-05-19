use std::{collections::HashMap, hash::Hash, thread, time::Duration};

struct Cacher<K, V, T>
where
    T: Fn(&K) -> V,
    K: Hash + Eq + Clone,
{
    calculation: T,
    cache: HashMap<K, V>,
}

impl<K, V, T> Cacher<K, V, T>
where
    T: Fn(&K) -> V,
    K: Hash + Eq + Clone,
{
    fn new(calculation: T) -> Cacher<K, V, T> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &K) -> &V {
        if !self.cache.contains_key(arg) {
            let key = arg.clone();
            let v = (self.calculation)(arg);
            self.cache.insert(key, v);
        }
        self.cache.get(arg).unwrap()
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|&num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(&intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
