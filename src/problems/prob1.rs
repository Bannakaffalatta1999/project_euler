// Library for euler problems

pub fn p() -> i32 {
    let mut results: Vec<i32> = vec![];
    let factor1 = 3;
    let factor2 = 5;
    let goal = 1000;
    for i in 1..goal {
        if i % factor1 == 0 || i % factor2 == 0 {
            results.push(i);
        }
    }
    let total: i32 = results.iter().sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution(){
        assert_eq!(p(), 233168);
    }
}