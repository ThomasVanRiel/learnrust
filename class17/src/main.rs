#[derive(Debug, Clone, Copy)]
struct Counter {
    count: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let val = self.count;
            self.count += 1;
            Some(val)
        } else {
            None
        }
    }
}

struct Fibbonacci {
    current: u64,
    next: u64,
}
impl Fibbonacci {
    // Constructor to provide default starting values
    fn new() -> Self {
        Fibbonacci {
            current: 0,
            next: 1,
        }
    }
}
impl Iterator for Fibbonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current;
        self.current = self.next;
        self.next = val + self.current;
        Some(val)
    }
}

fn main() {
    let counter: Counter = Counter { count: 0, max: 3 };
    println!("{:?}", counter.collect::<Vec<u32>>());
    println!("{:?}", counter.sum::<u32>());

    let iter = Counter { count: 0, max: 5 }
        .map(|x| {
            println!("mapping {x}");
            x * 2
        })
        .filter(|x| x > &4);
    println!("iterator created, nothing happened yet");
    let result: Vec<u32> = iter.collect(); // This wovrks too, without turbofish
    println!("{:?}", result);

    let result2: Vec<u32> = (0..).filter(|x| x % 3 == 0).take(5).collect();
    println!("{:?}", result2);

    let fib = Fibbonacci::new();
    let v_fib: Vec<u64> = fib.take(10).collect();
    println!("{:?}", v_fib);

    // fold
    let product = (1..=5).fold(1, |acc, x| acc * x);
    println!("Product: {product}");

    // flat_map
    let sentences = vec!["hello world", "foo bar baz"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split(" ")).collect();
    println!("Sentences: {:?}", sentences);
    println!("Words: {:?}", words);

    // zip
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![85, 92, 79];
    let highest = names
        .iter()
        .zip(scores)
        .max_by_key(|(_, score)| *score)
        .unwrap();
    println!("{:?}", highest);
}
