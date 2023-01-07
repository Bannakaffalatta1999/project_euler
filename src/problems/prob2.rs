// Problem 2
// Even Fibonacci numbers

pub fn p () -> i32 {

    struct Fib {
        last: i32,
        current: i32,
    }

    impl Fib {
        fn solve_next (self: &mut Fib) -> Fib {
            let next = self.current + self.last;
            self.last = self.current;
            self.current = next;
            Fib {last: self.last, current: self.current}
        }
    }

    let mut seed = Fib {last: 1, current: 1};

    let mut even_fibs: Vec<i32> = vec![];

    loop {
        seed.solve_next();
        if seed.current > 4000000 {
            break;
        } else if seed.current % 2 == 0 {
            even_fibs.push(seed.current);
        }
    }
    let total: i32 = even_fibs.iter().sum();

    total
}