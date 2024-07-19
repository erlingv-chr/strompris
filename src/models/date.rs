use chrono::NaiveDate;
use chrono::{Datelike, IsoWeek, Weekday};
use derive_more::Display;

/// A wrapper for chrono::NaiveDate.
///
/// A convenient (but optional) way of using the library without adding chrono to your
/// Cargo.toml file.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Display, Default)]
pub struct Date {
    inner_date: NaiveDate,
}

impl Date {
    /// Shadows the [`chrono::NaiveDate::from_ymd_opt`] method.
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Self> {
        let naive_date = NaiveDate::from_ymd_opt(year, month, day);
        naive_date.map(|d| Date { inner_date: d} )
    }
}

impl Datelike for Date {
    fn year(&self) -> i32 {
        self.inner_date.year()
    }

    fn month(&self) -> u32 {
        self.inner_date.month()
    }

    fn month0(&self) -> u32 {
        self.inner_date.month0()
    }

    fn day(&self) -> u32 {
        self.inner_date.day()
    }

    fn day0(&self) -> u32 {
        self.inner_date.day0()
    }

    fn ordinal(&self) -> u32 {
        self.inner_date.ordinal()
    }

    fn ordinal0(&self) -> u32 {
        self.inner_date.ordinal0()
    }

    fn weekday(&self) -> Weekday {
        self.inner_date.weekday()
    }

    fn iso_week(&self) -> IsoWeek {
        self.inner_date.iso_week()
    }

    fn with_year(&self, year: i32) -> Option<Self> {
        self.inner_date.with_year(year).map(|d| Date { inner_date: d} )
    }

    fn with_month(&self, month: u32) -> Option<Self> {
        self.inner_date.with_month(month).map(|d| Date { inner_date: d} )
    }

    fn with_month0(&self, month0: u32) -> Option<Self> {
        self.inner_date.with_month0(month0).map(|d| Date { inner_date: d} )
    }

    fn with_day(&self, day: u32) -> Option<Self> {
        self.inner_date.with_day(day).map(|d| Date { inner_date: d} )
    }

    fn with_day0(&self, day0: u32) -> Option<Self> {
        self.inner_date.with_day0(day0).map(|d| Date { inner_date: d} )
    }

    fn with_ordinal(&self, ordinal: u32) -> Option<Self> {
        self.inner_date.with_ordinal(ordinal).map(|d| Date { inner_date: d} )
    }

    fn with_ordinal0(&self, ordinal0: u32) -> Option<Self> {
        self.inner_date.with_ordinal0(ordinal0).map(|d| Date { inner_date: d} )
    }
}
