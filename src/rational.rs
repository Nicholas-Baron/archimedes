use std::cmp;
use std::ops;

#[derive(Debug)]
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

impl ops::Add<Rational> for Rational {
    type Output = Rational;
    fn add(self, rhs: Rational) -> Rational {
        Rational {
            top: (self.top * rhs.bottom) + (self.bottom * rhs.top),
            bottom: self.bottom * rhs.bottom,
        }
    }
}

impl ops::Sub<Rational> for Rational {
    type Output = Rational;
    fn sub(self, rhs: Rational) -> Rational {
        Rational {
            top: (self.top * rhs.bottom) - (self.bottom * rhs.top),
            bottom: self.bottom * rhs.bottom,
        }
    }
}

impl ops::Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        if self.bottom < 0 {
            Rational {
                bottom: -self.bottom,
                ..self
            }
        } else {
            Rational {
                top: -self.top,
                ..self
            }
        }
    }
}

impl ops::Mul<Rational> for Rational {
    type Output = Rational;
    fn mul(self, rhs: Rational) -> Rational {
        Rational {
            top: self.top * rhs.top,
            bottom: self.bottom * rhs.bottom,
        }
    }
}

impl ops::Div<Rational> for Rational {
    type Output = Rational;
    fn div(self, rhs: Rational) -> Rational {
        Rational {
            top: self.top * rhs.bottom,
            bottom: self.bottom * rhs.top,
        }
    }
}

impl cmp::PartialEq for Rational {
    fn eq(&self, rhs: &Rational) -> bool {
        self.top * rhs.bottom == self.bottom * rhs.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_test() {
        assert!(Rational::new(1, 2) + Rational::new(1, 2) == Rational::from(1));
        assert!(Rational::new(1, 2) - Rational::new(1, 2) == Rational::from(0));
        assert!(Rational::new(1, 2) * Rational::new(1, 2) == Rational::new(1, 4));
        assert!(Rational::new(1, 2) / Rational::from(2) == Rational::new(1, 4));
    }
}
