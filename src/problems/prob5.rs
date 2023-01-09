// Problem 5
// Smallest multiple

pub fn p() {
    let mut possible_multiples: = (1..10).collect();

    for x in 1..10 {
        let mut new_factor = possible_multiples.get(x);

        new_factor = new_factor * x ;
    }
}