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

pub fn prime_factors(number: u64) -> Vec<u64> {
    let mut val = number;

    let mut to_ret = Vec::new();

    while val > 1 {
        if val % 2 == 0 {
            to_ret.push(2);
            val /= 2;
        } else {
            let mut rem = 3;
            while rem <= val && val % rem != 0 {
                rem += 2;
            }

            if rem <= val {
                to_ret.push(rem);
                val /= rem;
            }
        }
    }

    to_ret
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn factors_test() {
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

    #[test]
    fn prime_factors_test() {
        for num in 1..100 {
            let prime_factors = prime_factors(num);

            // Every number that is not 1 has at least 1 prime factor.
            // 1 has 0 prime factors
            if num != 1 {
                assert_ne!(prime_factors.len(), 0);
            } else {
                assert_eq!(prime_factors.len(), 0);
            }

            // All even numbers should have their first prime factor as 2
            if num % 2 == 0 {
                assert_eq!(prime_factors[0], 2);
            }

            // If a number is prime, it should be its own only factor
            match num {
                3 | 5 | 7 | 11 | 13 | 17 | 23 | 29 => {
                    assert_eq!(prime_factors.len(), 1);
                    assert_eq!(prime_factors[0], num);
                }
                _ => {}
            }
        }
    }
}
