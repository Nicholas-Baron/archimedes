use archimedes::factoring::prime_factors::prime_factors;
use archimedes::is_prime;

#[test]
fn all_prime_factors_are_prime() {
    for num in 1..100 {
        let prime_factors = prime_factors(num);

        for factor in prime_factors {
            assert!(is_prime(factor));
        }
    }
}
