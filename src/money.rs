// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
/// Exact money amount, stored in cents as integers. Never floating point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cents(i128);

impl Cents {
    /// Creates an amount from a count of cents.
    #[must_use]
    pub const fn new(cents: i128) -> Self {
        Self(cents)
    }

    /// Creates an amount from whole euros. Saturates at the `i128` limits instead of overflowing.
    #[must_use]
    pub const fn from_euros(euros: i128) -> Self {
        Self(euros.saturating_mul(100))
    }

    /// Returns the amount in cents.
    #[must_use]
    pub const fn get(self) -> i128 {
        self.0
    }

    /// Rounds down to the whole euro (towards negative infinity).
    #[must_use]
    pub const fn whole_euros(self) -> i128 {
        self.0.div_euclid(100)
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    // Over the full i128 range, from_euros is exact or saturates at the matching limit.
    #[kani::proof]
    fn from_euros_saturates_instead_of_overflowing() {
        let euros: i128 = kani::any();
        let cents = Cents::from_euros(euros).get();
        match euros.checked_mul(100) {
            Some(exact) => assert_eq!(cents, exact),
            None if euros > 0 => assert_eq!(cents, i128::MAX),
            None => assert_eq!(cents, i128::MIN),
        }
    }
}
