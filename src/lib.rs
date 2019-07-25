pub mod factoring;
pub mod rational;

pub fn is_prime(number: u64) -> bool {
    if number <= 14 {
        match number {
            1 | 9 => false,
            2 => true,
            _ => number % 2 == 1,
        }
    } else if number % 2 == 0 || number % 3 == 0 || number % 5 == 0 {
        false
    } else {
        let start_factor = 7;
        let mut factor = start_factor;

        while factor <= number / start_factor {
            if number % factor == 0 {
                return false;
            }
            factor += 2;
        }

        true
    }
}

pub fn gcd(a: u64, b: u64) -> u64 {
    assert!(a > b);
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_test() {
        for num in 1..100 {
            let prime = is_prime(num);

            // An even number that is not 2 is not prime
            if num % 2 == 0 && num != 2 {
                assert!(!prime);
            }
        }
    }

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(54, 24), 6);
    }
}
