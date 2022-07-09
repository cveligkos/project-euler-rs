fn main() {
    let mut i = 1;
    loop {
        if is_divisible(i) {
            println!("Found: {}", i);
            break;
        }

        i += 1;
    }
}

fn is_divisible(n: i32) -> bool {
    for i in (1..=20).rev() {
        // println!("i = {}", i);
        if n % i != 0 {
            return false;
        }
    }

    return true;
}
