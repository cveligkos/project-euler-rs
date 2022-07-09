fn main() {
    let mut max_palindrome = 0;
    for i in 100..=999 {
        for j in 100..=999 {
            let p = i * j;
            if is_palindrome(p) {
                if p > max_palindrome {
                    max_palindrome = p;
                }
                // println!("Found palindrome: {}", p);
            }
        }
    }

    println!("Max palidnrome is {}", max_palindrome);
}

fn is_palindrome(n: i32) -> bool {
    let digits = get_digits(n);
    let mut reverse_digits = digits.clone();
    reverse_digits.reverse();

    if digits == reverse_digits {
        return true;
    } else {
        return false;
    }
}

fn get_digits(n: i32) -> Vec<i32> {
    let mut digits = Vec::with_capacity(10);
    let mut tmp = n;

    while tmp > 0 {
        let digit = tmp % 10;
        digits.insert(0, digit);
        tmp /= 10;
    }

    // println!("Digits of {}: {:?}", n, digits);
    return digits;
}
