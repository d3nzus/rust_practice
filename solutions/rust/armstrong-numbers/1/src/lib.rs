pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut digits = Vec::new();

    if n == 0 {
        digits = vec![0];
    }
    while n > 0 {
        digits.push(n % 10);
        n = n / 10;
    }

    digits.reverse();
    
    let power = digits.len();

    let mut temp: u32 = 0;
    for i in digits{    
        temp = temp + i.pow(power.try_into().unwrap());
    }

    return num == temp;

}
