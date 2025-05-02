struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn inc(&mut self) {
        self.count += 1;
    }

    fn dec(&mut self) {
        self.count -= 1;
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn get_count(&self) -> u32 {
        self.count
    }

    fn get_count_mut(&mut self) -> &mut u32 {
        &mut self.count
    }

    fn get_count_ref(&self) -> &u32 {
        &self.count
    }

}

fn main() {
    let mut counter = Counter::new();

    println!("counter is {}", counter.count);

    counter.inc();
    println!("counter is {}", counter.count);

    counter.dec();
    println!("counter is {}", counter.count);

    counter.reset();
    println!("counter is {}", counter.count);

    let count = counter.get_count();
    println!("count is {}", count);

    let count_mut = counter.get_count_mut();
    println!("count_mut is {}", count_mut);

    let count_ref = counter.get_count_ref();
    println!("count_ref is {}", count_ref);
}
