use std::vec::Vec;

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
    fn main_test() {
        for num in 1..100 {
            let prime_factors = prime_factors(num);

            // Every number that is not 1 has at least 1 prime factor.
            // 1 has 0 prime factors
            if num != 1 {
                assert_ne!(prime_factors.len(), 0);
            } else {
                assert_eq!(prime_factors.len(), 0);
            }
        }
    }
}
