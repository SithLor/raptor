//
/// Represents the months of the year
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Month {
    /// January (month 1)
    JANUARY = 1,
    /// February (month 2)
    FEBRUARY,
    /// March (month 3)
    MARCH,
    /// April (month 4)
    APRIL,
    /// May (month 5)
    MAY,
    /// June (month 6)
    JUNE,
    /// July (month 7)
    JULY,
    /// August (month 8)
    AUGUST,
    /// September (month 9)
    SEPTEMBER,
    /// October (month 10)
    OCTOBER,
    /// November (month 11)
    NOVEMBER,
    /// December (month 12)
    DECEMBER,
}

impl Month {
    /// Get the month as a number (1-12)
    pub fn number(&self) -> u8 {
        *self as u8
    }
    
    /// Get the month from a number (1-12)
    /// Returns None if the number is out of range
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            1 => Some(Month::JANUARY),
            2 => Some(Month::FEBRUARY),
            3 => Some(Month::MARCH),
            4 => Some(Month::APRIL),
            5 => Some(Month::MAY),
            6 => Some(Month::JUNE),
            7 => Some(Month::JULY),
            8 => Some(Month::AUGUST),
            9 => Some(Month::SEPTEMBER),
            10 => Some(Month::OCTOBER),
            11 => Some(Month::NOVEMBER),
            12 => Some(Month::DECEMBER),
            _ => None,
        }
    }
    
    /// Get the name of the month
    pub fn name(&self) -> &'static str {
        match self {
            Month::JANUARY => "January",
            Month::FEBRUARY => "February",
            Month::MARCH => "March",
            Month::APRIL => "April",
            Month::MAY => "May",
            Month::JUNE => "June",
            Month::JULY => "July",
            Month::AUGUST => "August",
            Month::SEPTEMBER => "September",
            Month::OCTOBER => "October",
            Month::NOVEMBER => "November",
            Month::DECEMBER => "December",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_month_number() {
        assert_eq!(Month::JANUARY.number(), 1);
        assert_eq!(Month::DECEMBER.number(), 12);
    }
    
    #[test]
    fn test_from_number() {
        assert_eq!(Month::from_number(3), Some(Month::MARCH));
        assert_eq!(Month::from_number(0), None);
        assert_eq!(Month::from_number(13), None);
    }
    
    #[test]
    fn test_month_name() {
        assert_eq!(Month::JANUARY.name(), "January");
        assert_eq!(Month::DECEMBER.name(), "December");
    }
}