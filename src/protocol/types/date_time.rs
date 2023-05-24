use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

use crate::_unsafe::{RFC_DATE, RFC_TIME};
use crate::protocol::UCStr;

/// Error types, that can occur while constructing a [`Date`].
///
/// These error types will be used by [`InvalidDateError`] to denote the actual type or error, that
/// has occurred.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InvalidDateTypes {
    /// Returned if the given month is out of range.
    MonthOutOfRange,
    // Returned if the given day is out of range.
    DayOutOfRange,
    /// Returned if the year could not be parsed.
    InvalidYear,
    /// Returned if the month could not be parsed.
    InvalidMonth,
    /// Returned if the day could not be parsed.
    InvalidDay,
}

/// Error denoting, that an invalid date has been passed to the [`Date`] constructor.
///
/// This error will be raised by the constructor of the [`Date`] struct and
/// denotes, that either the year, the month, or the day had not been in the correct range
/// and thus, that the date defined was invalid.
///
/// The `error_type` will be one of the different [`InvalidDateTypes`] and contains the
/// actual error that has occurred. The `value` contains the corresponding value, that has
/// been passed to the constructor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InvalidDateError {
    pub error_type: InvalidDateTypes,
    pub value: String,
    pub max_value: u8,
}

impl fmt::Display for InvalidDateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Date: ")?;
        match self.error_type {
            InvalidDateTypes::MonthOutOfRange => {
                write!(
                    f,
                    "Month ({}) out of range (1-{})",
                    self.value, self.max_value
                )
            }
            InvalidDateTypes::DayOutOfRange => {
                write!(
                    f,
                    "Day ({}) out of range (1-{})",
                    self.value, self.max_value
                )
            }
            InvalidDateTypes::InvalidYear => {
                write!(f, "Invalid Year ({})", self.value)
            }
            InvalidDateTypes::InvalidMonth => {
                write!(f, "Invalid Month ({})", self.value)
            }
            InvalidDateTypes::InvalidDay => {
                write!(f, "Invalid Day ({})", self.value)
            }
        }
    }
}
impl Error for InvalidDateError {}

fn is_leap_year(year: u32) -> bool {
    // A leap year must match the following criteria:
    // 1. It must be dividable by `4`.
    // 2. It must either not be dividable by `100`, or it must be dividable by `100` and `400`.
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}

/// A struct representing a date.
///
/// This struct can be constructed using the [`new`] method and consists of a
/// year, a month and a day. During the construction, it will be checked, that the
/// different parts are all within valid ranges and a [`InvalidDateError`] will be raised
/// in case of an error.
///
/// [`new`]: Date::new
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Date {
    date: RFC_DATE,
}

impl Date {
    /// Construct a new date.
    ///
    /// This method constructs a new date from the given year, month and day.
    /// During the construction, it checks, that the month is within the range
    /// of 1-12 and that the day is valid for the given month and year
    /// (e.g the 29th february in leap years).
    /// In case of an error as [`InvalidDateError`] will be returned, describing
    /// the error.
    ///
    /// # Examples
    /// ```
    /// use nwrfc::protocol::Date;
    ///
    /// let date = Date::new(2023, 06, 16).expect("Invalid date!");
    /// println!("{}", date)
    /// ```
    ///
    /// ```
    /// use nwrfc::protocol::Date;
    ///
    /// Date::new(2023, 02, 30).unwrap_err();
    /// ```
    pub fn new(year: u32, month: u8, day: u8) -> Result<Self, InvalidDateError> {
        // Check the month
        if month < 1 || month > 12 {
            return Err(InvalidDateError {
                error_type: InvalidDateTypes::MonthOutOfRange,
                value: month.to_string(),
                max_value: 12,
            });
        }
        // Check the day.
        if month != 2 {
            // If the month is not february, check whether it has 30 or 31 days.
            // Uneven month have 31 day, even ones 30.
            if month % 2 == 0 && (day < 1 || day > 30) {
                return Err(InvalidDateError {
                    error_type: InvalidDateTypes::DayOutOfRange,
                    value: day.to_string(),
                    max_value: 30,
                });
            } else if day < 1 || day > 31 {
                return Err(InvalidDateError {
                    error_type: InvalidDateTypes::DayOutOfRange,
                    value: day.to_string(),
                    max_value: 31,
                });
            }
        } else {
            match is_leap_year(year) {
                true => {
                    if day < 1 || day > 29 {
                        return Err(InvalidDateError {
                            error_type: InvalidDateTypes::DayOutOfRange,
                            value: day.to_string(),
                            max_value: 29,
                        });
                    }
                }
                false => {
                    if day < 1 || day > 28 {
                        return Err(InvalidDateError {
                            error_type: InvalidDateTypes::DayOutOfRange,
                            value: day.to_string(),
                            max_value: 28,
                        });
                    }
                }
            }
        }
        // Write the date into a string
        let s = format!("{:04}{:02}{:02}", year, month, day);
        let mut date = RFC_DATE::default();
        UCStr::from_slice_mut(&mut date)
            .write_without_nul(s)
            .unwrap();
        Ok(Self { date })
    }

    /// Get the year of the date.
    #[inline]
    pub fn year(&self) -> u32 {
        let s = UCStr::from_slice(&self.date).to_string_lossy();
        s[0..4].parse::<u32>().expect("Invalid year in date!")
    }

    /// Get the month of the date
    #[inline]
    pub fn month(&self) -> u8 {
        let s = UCStr::from_slice(&self.date).to_string_lossy();
        s[4..6].parse::<u8>().expect("Invalid month in date!")
    }

    /// Get the day of the date
    #[inline]
    pub fn day(&self) -> u8 {
        let s = UCStr::from_slice(&self.date).to_string_lossy();
        s[6..8].parse::<u8>().expect("Invalid day in date!")
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}",
            self.year(),
            self.month(),
            self.day()
        )
    }
}

impl TryFrom<RFC_DATE> for Date {
    type Error = InvalidDateError;

    /// Create a new `Date` from a given NetWeaver RFC Date.
    ///
    /// This method constructs a new date from the NetWeaver RFC Date.
    /// During construction, the RFC Date will be parsed and it checks,
    /// that the month is within the range of 1-12 and that the day is
    /// valid for the given month and year (e.g the 29th february in leap years).
    /// In case of an error as [`InvalidDateError`] will be returned, describing
    /// the error.
    ///
    /// # Examples
    /// ```
    /// use nwrfc::protocol::{Date, UCStr};
    /// use nwrfc::_unsafe::RFC_DATE;
    ///
    /// let mut rfc_date = RFC_DATE::default();
    /// UCStr::from_slice_mut(&mut rfc_date)
    ///     .write_without_nul("20230616")
    ///     .expect("Unable to write the date!");
    ///
    /// let date = Date::try_from(rfc_date).expect("Invalid date!");
    /// assert_eq!(2023, date.year());
    /// assert_eq!(6, date.month());
    /// assert_eq!(16, date.day());
    /// ```
    #[inline(always)]
    fn try_from(value: RFC_DATE) -> Result<Self, Self::Error> {
        let s = UCStr::from_slice(value.as_slice()).to_string_lossy();
        let year = s[0..4].parse::<u32>();
        let month = s[4..6].parse::<u8>();
        let day = s[6..8].parse::<u8>();
        match year {
            Err(_) => Err(InvalidDateError {
                error_type: InvalidDateTypes::InvalidYear,
                value: s[0..4].to_string(),
                max_value: 0,
            }),
            Ok(year) => match month {
                Err(_) => Err(InvalidDateError {
                    error_type: InvalidDateTypes::InvalidMonth,
                    value: s[4..6].to_string(),
                    max_value: 0,
                }),
                Ok(month) => match day {
                    Err(_) => Err(InvalidDateError {
                        error_type: InvalidDateTypes::InvalidDay,
                        value: s[6..8].to_string(),
                        max_value: 0,
                    }),
                    Ok(day) => Self::new(year, month, day),
                },
            },
        }
    }
}
impl From<Date> for RFC_DATE {
    fn from(value: Date) -> Self {
        value.date
    }
}

/// Error types, that can occur while constructing a [`Time`].
///
/// These error types will be used by [`InvalidTimeError`] to denote the actual type or error, that
/// has occurred.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InvalidTimeTypes {
    /// Returned if the given hour are out of range.
    HourOutOfRange,
    /// Returned if the given minute are out of range.
    MinuteOutOfRange,
    /// Returned if the given second are out of range.
    SecondOutOfRange,
    /// Returned if the hour could not be parsed.
    InvalidHour,
    /// Returned if the minute could not be parsed.
    InvlidMinute,
    /// Returned if the second could not be parsed.
    InvlidSecond,
}

/// Error denoting, that an invalid time has been passed to the [`Time`] constructor.
///
/// This error will be raised by the constructor of the [`Time`] struct and
/// denotes, that either the hours, the minutes, or the seconds had not been in the correct range
/// and thus, that the time defined was invalid.
///
/// The `error_type` will be one of the different [`InvalidTimeTypes`] and contains the
/// actual error that has occurred. The `value` contains the corresponding value, that has
/// been passed to the constructor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InvalidTimeError {
    pub error_type: InvalidTimeTypes,
    pub value: String,
}
impl fmt::Display for InvalidTimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Time: ")?;
        match self.error_type {
            InvalidTimeTypes::HourOutOfRange => {
                write!(f, "Hour ({}) out of range (0-23)", self.value)
            }
            InvalidTimeTypes::MinuteOutOfRange => {
                write!(f, "Minute ({}) out of range (0-59)", self.value)
            }
            InvalidTimeTypes::SecondOutOfRange => {
                write!(f, "Second ({}) out of range (0-59)", self.value)
            }
            InvalidTimeTypes::InvalidHour => {
                write!(f, "Invalid hour value ({})", self.value)
            }
            InvalidTimeTypes::InvlidMinute => {
                write!(f, "Invalid minute value ({})", self.value)
            }
            InvalidTimeTypes::InvlidSecond => {
                write!(f, "Invalid second value ({})", self.value)
            }
        }
    }
}
impl Error for InvalidTimeError {}

/// A struct representing a point in time on a day.
///
/// This struct can be constructed using the [`new`] method and consists of an hour, a minute and
/// a second. During the construction, it will be checked, that the
/// different parts are all within valid ranges and a [`InvalidTimeError`] will be raised
/// in case of an error.
///
/// [`new`]: Time::new
///
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    /// Construct a new time.
    ///
    /// This method constructs a new time from the given hour, minute, and second.
    /// During the construction, it checks, that the hour is within the range
    /// of 0-23 and that the minute and second are within the range or 0-59.
    /// In case of an error as [`InvalidTimeError`] will be returned, describing
    /// the error.
    pub fn new(hour: u8, minute: u8, second: u8) -> Result<Self, InvalidTimeError> {
        if hour > 23 {
            return Err(InvalidTimeError {
                error_type: InvalidTimeTypes::HourOutOfRange,
                value: hour.to_string(),
            });
        }
        if minute > 59 {
            return Err(InvalidTimeError {
                error_type: InvalidTimeTypes::MinuteOutOfRange,
                value: minute.to_string(),
            });
        }
        if second > 59 {
            return Err(InvalidTimeError {
                error_type: InvalidTimeTypes::SecondOutOfRange,
                value: second.to_string(),
            });
        }
        Ok(Self {
            hour,
            minute,
            second,
        })
    }

    /// Get the hour of the day.
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Get the minute of the day.
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Get the second of the day.
    pub fn second(&self) -> u8 {
        self.second
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl TryFrom<RFC_TIME> for Time {
    type Error = InvalidTimeError;

    fn try_from(value: RFC_TIME) -> Result<Self, Self::Error> {
        let s = UCStr::from_slice(&value).to_string_lossy();
        let hour = s[0..2].parse::<u8>();
        let minute = s[2..4].parse::<u8>();
        let second = s[4..6].parse::<u8>();
        match hour {
            Err(_) => Err(InvalidTimeError {
                error_type: InvalidTimeTypes::InvalidHour,
                value: s[0..2].to_string(),
            }),
            Ok(hour) => match minute {
                Err(_) => Err(InvalidTimeError {
                    error_type: InvalidTimeTypes::InvlidMinute,
                    value: s[2..4].to_string(),
                }),
                Ok(minute) => match second {
                    Err(_) => Err(InvalidTimeError {
                        error_type: InvalidTimeTypes::InvlidSecond,
                        value: s[4..6].to_string(),
                    }),
                    Ok(second) => Self::new(hour, minute, second),
                },
            },
        }
    }
}

impl From<Time> for RFC_TIME {
    fn from(value: Time) -> Self {
        let s = format!("{:02}{:02}{:02}", value.hour, value.minute, value.second);
        let mut result = Self::default();
        UCStr::from_slice_mut(&mut result)
            .write_without_nul(s)
            .unwrap();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_from_rfc_date() {
        let mut rfc_date = RFC_DATE::default();
        UCStr::from_slice_mut(&mut rfc_date)
            .write_without_nul("20230524")
            .expect("Could not write the string to RFC_DATE");
        let date = Date::try_from(rfc_date).expect("Could not parse the RFC_DATE");
        assert_eq!(date.day(), 24);
        assert_eq!(date.month(), 5);
        assert_eq!(date.year(), 2023);
    }

    #[test]
    fn test_date_from_leap_day() {
        let mut rfc_date = RFC_DATE::default();
        UCStr::from_slice_mut(&mut rfc_date)
            .write_without_nul("20200229")
            .expect("Could not write the string to RFC_DATE");
        let date = Date::try_from(rfc_date).expect("Could not parse the RFC_DATE");
        assert_eq!(date.day(), 29);
        assert_eq!(date.month(), 2);
        assert_eq!(date.year(), 2020);
    }

    #[test]
    fn test_date_from_invalid_rfc_date() {
        let mut invalid_rfc_date = RFC_DATE::default();
        // Write invalid day
        UCStr::from_slice_mut(&mut invalid_rfc_date)
            .write_without_nul("13371237")
            .expect("Could not write the string to RFC_DATE");
        Date::try_from(invalid_rfc_date).expect_err("Could construct from invalid date!");
        // Write invalid month
        UCStr::from_slice_mut(&mut invalid_rfc_date)
            .write_without_nul("13371301")
            .expect("Could not write the string to RFC_DATE");
        Date::try_from(invalid_rfc_date).expect_err("Could construct from invalid date!");
        // Write invalid leap day
        UCStr::from_slice_mut(&mut invalid_rfc_date)
            .write_without_nul("21000229")
            .expect("Could not write the string to RFC_DATE");
        Date::try_from(invalid_rfc_date).expect_err("Could construct from invalid date!");
        // Write garbage data
        UCStr::from_slice_mut(&mut invalid_rfc_date)
            .write_without_nul("INVALID!")
            .expect("Could not wirte the stirng to RFC_DATE");
        Date::try_from(invalid_rfc_date).expect_err("Could construct from invalid date!");
    }

    #[test]
    fn test_time_from_rfc_time() {
        let mut rfc_time = RFC_TIME::default();
        UCStr::from_slice_mut(&mut rfc_time)
            .write_without_nul("133742")
            .expect("Could not write to RFC_TIME");
        let time = Time::try_from(rfc_time).expect("Could not parse RFC_TIME");
        assert_eq!(time.hour(), 13);
        assert_eq!(time.minute(), 37);
        assert_eq!(time.second(), 42);
    }

    #[test]
    fn test_time_from_invalid_rfc_time() {
        let mut rfc_time = RFC_TIME::default();
        // Write invalid hour
        UCStr::from_slice_mut(&mut rfc_time)
            .write_without_nul("250000")
            .expect("Could not write to RFC_TIME");
        Time::try_from(rfc_time).expect_err("Could parse from invalid hour");
        // Write invalid minute
        UCStr::from_slice_mut(&mut rfc_time)
            .write_without_nul("006100")
            .expect("Could not write to RFC_TIME");
        Time::try_from(rfc_time).expect_err("Could parse from invalid minute");
        // Write invalid second
        UCStr::from_slice_mut(&mut rfc_time)
            .write_without_nul("000073")
            .expect("Could not write to RFC_TIME");
        Time::try_from(rfc_time).expect_err("Could parse from invalid second");
        // Write garbage data
        UCStr::from_slice_mut(&mut rfc_time)
            .write_without_nul("invali")
            .expect("Could not write to RFC_TIME");
        Time::try_from(rfc_time).expect_err("Could parse from garbage data");
    }
}
