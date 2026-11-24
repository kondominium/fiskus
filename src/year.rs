// Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
// Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
/// Assessment year (Veranlagungszeitraum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxYear {
    /// Assessment year 2003 (VZ 2003).
    Y2003,
    /// Assessment year 2004 (VZ 2004).
    Y2004,
    /// Assessment year 2005 (VZ 2005).
    Y2005,
    /// Assessment year 2006 (VZ 2006).
    Y2006,
    /// Assessment year 2007 (VZ 2007).
    Y2007,
    /// Assessment year 2008 (VZ 2008).
    Y2008,
    /// Assessment year 2009 (VZ 2009).
    Y2009,
    /// Assessment year 2010 (VZ 2010).
    Y2010,
    /// Assessment year 2011 (VZ 2011).
    Y2011,
    /// Assessment year 2012 (VZ 2012).
    Y2012,
    /// Assessment year 2013 (VZ 2013).
    Y2013,
    /// Assessment year 2014 (VZ 2014).
    Y2014,
    /// Assessment year 2015 (VZ 2015).
    Y2015,
    /// Assessment year 2016 (VZ 2016).
    Y2016,
    /// Assessment year 2017 (VZ 2017).
    Y2017,
    /// Assessment year 2018 (VZ 2018).
    Y2018,
    /// Assessment year 2019 (VZ 2019).
    Y2019,
    /// Assessment year 2020 (VZ 2020).
    Y2020,
    /// Assessment year 2021 (VZ 2021).
    Y2021,
    /// Assessment year 2022 (VZ 2022).
    Y2022,
    /// Assessment year 2023 (VZ 2023).
    Y2023,
    /// Assessment year 2024 (VZ 2024).
    Y2024,
    /// Assessment year 2025 (VZ 2025).
    Y2025,
    /// Assessment year 2026 (VZ 2026).
    Y2026,
}

impl TaxYear {
    /// Calendar year of the assessment year, so `TaxYear::Y2026.year()` is 2026.
    pub const fn year(self) -> u16 {
        match self {
            Self::Y2003 => 2003,
            Self::Y2004 => 2004,
            Self::Y2005 => 2005,
            Self::Y2006 => 2006,
            Self::Y2007 => 2007,
            Self::Y2008 => 2008,
            Self::Y2009 => 2009,
            Self::Y2010 => 2010,
            Self::Y2011 => 2011,
            Self::Y2012 => 2012,
            Self::Y2013 => 2013,
            Self::Y2014 => 2014,
            Self::Y2015 => 2015,
            Self::Y2016 => 2016,
            Self::Y2017 => 2017,
            Self::Y2018 => 2018,
            Self::Y2019 => 2019,
            Self::Y2020 => 2020,
            Self::Y2021 => 2021,
            Self::Y2022 => 2022,
            Self::Y2023 => 2023,
            Self::Y2024 => 2024,
            Self::Y2025 => 2025,
            Self::Y2026 => 2026,
        }
    }

    /// Returns the variant for `year` if fiskus implements that assessment year, otherwise `None`.
    pub fn from_year(year: u16) -> Option<Self> {
        match year {
            2003 => Some(Self::Y2003),
            2004 => Some(Self::Y2004),
            2005 => Some(Self::Y2005),
            2006 => Some(Self::Y2006),
            2007 => Some(Self::Y2007),
            2008 => Some(Self::Y2008),
            2009 => Some(Self::Y2009),
            2010 => Some(Self::Y2010),
            2011 => Some(Self::Y2011),
            2012 => Some(Self::Y2012),
            2013 => Some(Self::Y2013),
            2014 => Some(Self::Y2014),
            2015 => Some(Self::Y2015),
            2016 => Some(Self::Y2016),
            2017 => Some(Self::Y2017),
            2018 => Some(Self::Y2018),
            2019 => Some(Self::Y2019),
            2020 => Some(Self::Y2020),
            2021 => Some(Self::Y2021),
            2022 => Some(Self::Y2022),
            2023 => Some(Self::Y2023),
            2024 => Some(Self::Y2024),
            2025 => Some(Self::Y2025),
            2026 => Some(Self::Y2026),
            _ => None,
        }
    }

    /// Every implemented assessment year, oldest first.
    pub const ALL: [TaxYear; 24] = [
        TaxYear::Y2003,
        TaxYear::Y2004,
        TaxYear::Y2005,
        TaxYear::Y2006,
        TaxYear::Y2007,
        TaxYear::Y2008,
        TaxYear::Y2009,
        TaxYear::Y2010,
        TaxYear::Y2011,
        TaxYear::Y2012,
        TaxYear::Y2013,
        TaxYear::Y2014,
        TaxYear::Y2015,
        TaxYear::Y2016,
        TaxYear::Y2017,
        TaxYear::Y2018,
        TaxYear::Y2019,
        TaxYear::Y2020,
        TaxYear::Y2021,
        TaxYear::Y2022,
        TaxYear::Y2023,
        TaxYear::Y2024,
        TaxYear::Y2025,
        TaxYear::Y2026,
    ];
}
