pub fn p() -> i64 {
    let mut num: i64 = 600851475143;
    let mut denom = 2;
    let result: i64;

    loop {
        if num == denom {
            result = denom;
            break;

        } else if num % denom == 0 {
            num = num / denom;

        } else {
            denom += 1;
        }
    }

    result
}