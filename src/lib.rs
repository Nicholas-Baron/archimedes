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
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn quadratic_formula(a: f32, b: f32, c: f32) -> (f32, f32) {
    let radical = (b * b - a * c * 4.0).sqrt();
    let bottom = 2.0 * a;

    ((-b + radical) / bottom, (-b - radical) / bottom)
}

pub fn coprimes(val: u64) -> Vec<u64> {
    use factoring::prime_factors;
    let input_factors = prime_factors(val);

    let possibles = 2..val;
    let guesses = possibles.map(|val| (val, prime_factors(val)));

    let correct_guesses = guesses.filter(|(_, factors)| {
        for factor in &input_factors {
            if factors.contains(&factor) {
                return false;
            }
        }

        true
    });

    correct_guesses.map(|(guess, _)| guess).collect()
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

    #[test]
    fn quadratic_test() {
        assert_eq!(quadratic_formula(2.0, -5.0, -3.0), (3.0, -0.5));
    }

    #[test]
    fn coprime_test() {
        assert_eq!(coprimes(9), vec![2, 4, 5, 7, 8]);
    }
}
