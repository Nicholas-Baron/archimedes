use std::vec::Vec;

pub fn factors(number: u64) -> Vec<u64> {
    let mut to_ret = Vec::new();

    for factor in 1..number {
        if number % factor == 0 {
            to_ret.push(factor);
        }
    }

    to_ret.push(number);

    to_ret
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn main_test() {
        for num in 1..100 {
            let factors = factors(num);

            // Every list of factors is 1 or more
            assert_ne!(factors.len(), 0);

            // Every list of factors starts at 1
            assert_eq!(factors[0], 1);

            // If a number is even, the second number in its list of factors is 2
            if num % 2 == 0 {
                assert_eq!(factors[1], 2);
            }

            // Some numbers have a specific number of factors
            match num {
                1 | 2 => assert_eq!(factors.len() as u64, num), // 1 has 1 factor (1) and 2 has 2 factors (1, 2)
                3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 => assert_eq!(factors.len(), 2), // primes have two factors
                _ => {}
            }
        }
    }

}
