fn main() {
    let even_terms = EvenFibonacciTerms::new();
    let mut sum: u64 = 0;

    for x in even_terms {
        if x > 4_000_000 {
            break;
        } 
        sum += x;
    }

    println!("Even sum: {}", sum);
}

struct FibonacciSequence {
    previous: u64,
    current: u64,
}

impl FibonacciSequence {
    pub fn new() -> Self {
        Self { previous: 0, current: 1 }
    }
}

impl Iterator for FibonacciSequence {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let temp = self.previous;
        self.previous = self.current;
        self.current += temp;
        return Some(temp);
    }
}

struct EvenFibonacciTerms {
    sequence: FibonacciSequence
}

impl EvenFibonacciTerms {
    fn new() -> Self {
        Self {
            sequence: FibonacciSequence::new()
        }
    }
}

impl Iterator for EvenFibonacciTerms {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        
        loop {
           let current = self.sequence.next().unwrap();

            if current % 2 == 0 {
                return Some(current);
            }
        } 
    }
}
