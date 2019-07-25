pub struct Rational {
    top: i64,
    bottom: i64,
}

impl Rational {
    pub fn new(top: i64, bottom: i64) -> Rational {
        assert_ne!(bottom, 0);
        Rational { top, bottom }
    }
}

impl From<i64> for Rational {
    fn from(value: i64) -> Rational {
        Rational::new(value, 1)
    }
}
