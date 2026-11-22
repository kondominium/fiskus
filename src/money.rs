// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
/// Exact money amount, stored in cents as integers. Never floating point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cents(i128);

impl Cents {
    /// Creates an amount from a count of cents.
    pub const fn new(cents: i128) -> Self {
        Self(cents)
    }

    /// Creates an amount from whole euros. Saturates at the `i128` limits instead of overflowing.
    pub const fn from_euros(euros: i128) -> Self {
        Self(euros.saturating_mul(100))
    }

    /// Returns the amount in cents.
    pub const fn get(self) -> i128 {
        self.0
    }

    /// Rounds down to the whole euro (towards negative infinity).
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

    // whole_euros is floor division by 100, for every cents value in a bound.
    #[kani::proof]
    fn whole_euros_is_floor_division() {
        const BOUND: i128 = 1_000_000_000_000;
        let cents: i128 = kani::any();
        kani::assume(cents >= -BOUND && cents <= BOUND);
        let c = Cents::new(cents);
        assert_eq!(c.whole_euros(), cents.div_euclid(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn new_and_get_round_trip() {
        assert_eq!(Cents::new(12345).get(), 12345);
        assert_eq!(Cents::new(0).get(), 0);
        assert_eq!(Cents::new(-1).get(), -1);
    }

    #[test]
    fn from_euros_multiplies_by_100() {
        assert_eq!(Cents::from_euros(1).get(), 100);
        assert_eq!(Cents::from_euros(0).get(), 0);
        assert_eq!(Cents::from_euros(-1).get(), -100);
        assert_eq!(Cents::from_euros(12_348).get(), 1_234_800);
    }

    #[test]
    fn whole_euros_floors_towards_negative_infinity() {
        assert_eq!(Cents::new(123_4899).whole_euros(), 12_348); // 12348.99 -> 12348
        assert_eq!(Cents::new(-100).whole_euros(), -1); // -1.00 -> -1
        assert_eq!(Cents::new(-1).whole_euros(), -1); // -0.01 -> -1 (floor)
        assert_eq!(Cents::new(99).whole_euros(), 0);
    }

    #[test]
    fn ordering_and_equality() {
        assert!(Cents::new(100) > Cents::new(99));
        assert_eq!(Cents::new(5), Cents::new(5));
        assert_ne!(Cents::new(5), Cents::new(6));
    }

    proptest! {
        #[test]
        fn from_euros_round_trips_for_small_values(e in -1_000_000i128..=1_000_000i128) {
            prop_assert_eq!(Cents::from_euros(e).get(), e * 100);
        }

        #[test]
        fn whole_euros_is_floor(cents in any::<i128>()) {
            prop_assert_eq!(Cents::new(cents).whole_euros(), cents.div_euclid(100));
        }

        #[test]
        fn whole_euros_monotonic(a in any::<i128>(), b in any::<i128>()) {
            if a <= b {
                prop_assert!(Cents::new(a).whole_euros() <= Cents::new(b).whole_euros());
            }
        }
    }
}
