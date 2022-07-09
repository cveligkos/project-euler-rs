fn main() {
    let mut n: u64 = 600851475143;

    'outer: loop {
        let primes = PrimeNumbers::new();
        for i in primes {
            if n % i == 0 {
                println!("Current: {}", n);

                if n / i == 1 {
                    break 'outer;
                } else {
                    n = n / i;
                }
            }
        }
    }
    println!("Largest prime factor: {}", n);
}

struct PrimeNumbers {
    current: u64,
}

impl PrimeNumbers {
    pub fn new() -> Self {
        PrimeNumbers { current: 1 }
    }
}

impl Iterator for PrimeNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.current + 1;
        loop {
            if is_prime(next) {
                self.current = next;
                return Some(next);
            } else {
                next += 1;
            }
        }
    }
}

fn is_prime(n: u64) -> bool {
    let mut is_prime = true;

    for i in 2..n {
        if n % i == 0 {
            is_prime = false;
            break;
        }
    }

    return is_prime;
}
