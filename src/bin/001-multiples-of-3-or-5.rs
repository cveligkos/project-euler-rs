fn main() {
    let mut sum: u64 = 0;
    const LIMIT: u64 = 1000;

    for i in 1..LIMIT {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("The sum is: {}", sum);
}