use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    value: Option<String>,
    computation: T,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        Self {
            value: None,
            computation,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        if let Some(ref result) = self.value {
            result.clone()
        } else {
            let result = (self.computation)();
            self.value = Some(result.clone());
            result
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}