pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut result: Vec<u64> = Vec::new();

    // handle factors of 2
    while num % 2 == 0 {
        result.push(2);
        num /= 2;
    }

    // handle odd factors from 3 upward
    let mut i: u64 = 3;
    while i * i <= num {
        while num % i == 0 {
            result.push(i);
            num /= i;
        }
        i += 2;
    }

    // leftover prime, if any
    if num > 2 {
        result.push(num);
    }

    result
}