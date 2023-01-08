pub fn p() -> i32 {
    fn is_palendrome(n: i32) -> bool {
        let string_number_rev = n
            .to_string().chars().rev().collect::<String>();
        let string_number = n.to_string();

        if string_number_rev == string_number {
            return true
        } else {
            return false
        }
    }

    let mut largest = 0;
    for x in 100..999 {
        for y in 100..999 {
            let possible_pal = x * y;
            if is_palendrome(possible_pal) && possible_pal > largest{
                 largest = possible_pal;
            }
        }
    }
    largest
}