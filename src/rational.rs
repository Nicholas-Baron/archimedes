use std::cmp::{self, Ordering};
use std::ops;

use super::gcd;

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

    pub fn simplified(&self) -> Rational {
        match gcd(self.top.abs() as u64, self.bottom.abs() as u64) {
            0 | 1 => Rational { ..*self },
            n => Rational {
                top: self.top / n as i64,
                bottom: self.bottom / n as i64,
            },
        }
    }

    pub fn numerator(&self) -> i64 {
        self.top
    }

    pub fn denominator(&self) -> i64 {
        self.bottom
    }

    pub fn flip_signs(&self) -> Rational {
        let bottom = if self.numerator() < 0 {
            -self.denominator()
        } else {
            self.denominator()
        };

        let top = if self.denominator() < 0 {
            -self.numerator()
        } else {
            self.numerator()
        };

        Rational { top, bottom }
    }
}

impl Default for Rational {
    fn default() -> Rational {
        Rational { top: 0, bottom: 1 }
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

impl ops::AddAssign for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        *self = Rational {
            top: (self.top * rhs.bottom) + (self.bottom * rhs.top),
            bottom: self.bottom * rhs.bottom,
        };
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

impl cmp::Eq for Rational {}

impl cmp::PartialOrd for Rational {
    fn partial_cmp(&self, rhs: &Rational) -> Option<Ordering> {
        let lhs_use = if self.denominator() < 0 {
            self.flip_signs().simplified()
        } else {
            self.simplified()
        };

        let rhs_use = if rhs.denominator() < 0 {
            rhs.flip_signs().simplified()
        } else {
            rhs.simplified()
        };

        if rhs_use == lhs_use {
            Some(Ordering::Equal)
        } else {
            (lhs_use.numerator() * rhs_use.denominator())
                .partial_cmp(&(lhs_use.denominator() * rhs_use.numerator()))
        }
    }
}

impl cmp::Ord for Rational {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_test() {
        // Math ops
        assert!(Rational::new(1, 2) + Rational::new(1, 2) == Rational::from(1));
        assert!(Rational::new(1, 2) - Rational::new(1, 2) == Rational::from(0));
        assert!(Rational::new(1, 2) * Rational::new(1, 2) == Rational::new(1, 4));
        assert!(Rational::new(1, 2) / Rational::from(2) == Rational::new(1, 4));

        // Comparisons
        assert!(Rational::new(1, 3) < Rational::new(1, 2));
        assert!(Rational::from(1) > Rational::new(1, 2));
    }
}
