// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
/// Exact amount in cents. Never floating point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cents(i128);

impl Cents {
    pub const fn new(cents: i128) -> Self {
        Self(cents)
    }

    pub const fn from_euros(euros: i128) -> Self {
        Self(euros.saturating_mul(100))
    }

    pub const fn get(self) -> i128 {
        self.0
    }

    /// Rounds down to the whole euro (towards negative infinity).
    pub const fn whole_euros(self) -> i128 {
        self.0.div_euclid(100)
    }
}
