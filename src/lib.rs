//! This crate offers a wrapper of the Strømpris API offered by HvaKosterStrømmen.
//!
//! The crate is designed to be as simple as the API itself, so only one method is exposed: A
//! method for getting the prices for a given region on a given day.
//!
//! This crate offers both async and blocking ways of fetching prices. See the blocking module
//! for more information on the blocking API.
//!
//! See [`www.hvakosterstrommen.no`] for more info about the API.
//!
//! Example using tokio:
//! ```rust
//! use strompris::{Strompris, PriceRegion, Date, Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let date = Date::from_ymd_opt(2024, 1, 31).unwrap();
//!     let client = Strompris::default();
//!     let prices = client.get_prices(date, PriceRegion::NO1).await?;
//!     for price in prices.iter() {
//!         println!("Price: {:.2}", price.nok_per_kwh);
//!         println!("From: {}", price.time_start.time());
//!         println!("To: {}", price.time_end.time());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! [`www.hvakosterstrommen.no`]: www.hvakosterstrommen.no

#![deny(missing_docs)]

use crate::error::Result;
use chrono::{Datelike, NaiveDate};
use reqwest::header::HeaderMap;
use reqwest::Client;
use url::Url;

pub use error::Error;
pub use models::Date;
pub use models::HourlyPrice;
pub use models::PriceRegion;

pub mod blocking;
pub mod error;
mod local_time_deserializer;
mod models;

// Has to be an option because of rustc limitations.
static MIN_DATE: Option<NaiveDate> = NaiveDate::from_ymd_opt(2021, 12, 1);

/// The client for communicating with the Strømpris API hosted on
/// [`www.hvakosterstrommen.no`].
///
/// It exposes a single method for communicating with the API.
///
/// Example:
/// ```rust
/// # use strompris::{PriceRegion, Strompris, Date, Error};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Error> {
/// use strompris::Error;
/// let date = Date::from_ymd_opt(2024, 7, 14).unwrap();
/// let client = Strompris::default();
/// let resp = client.get_prices(date, PriceRegion::NO1).await?;
/// for r in resp.iter() {
///     dbg!(r);
/// }
/// # Ok(())
/// # }
/// ```
/// [`www.hvakosterstrommen.no`]: www.hvakosterstrommen.no
pub struct Strompris {
    client: Client,
    base_url: Url,
}

impl Strompris {
    fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        let client = Client::builder()
            .default_headers(headers)
            .https_only(true)
            .user_agent("Strømpris API wrapper written in Rust, github.com/erlingv-chr")
            .build()
            .unwrap();
        let base_url = Url::parse("https://www.hvakosterstrommen.no/api/v1/prices/").unwrap();
        Strompris { client, base_url }
    }

    /// Get the prices for the given date and price region.
    ///
    /// The prices are represented by a vector consisting of 24 hourly prices.
    ///
    /// Note: The API does not know the future! Tomorrow's prices are usually ready by 13:00,
    /// local time.
    pub async fn get_prices(&self, date: impl Datelike, price_region: PriceRegion) -> Result<Vec<HourlyPrice>> {
        if !self.date_after_min_date(&date) {
            return Err(Error::Generic("Date is before the minimum acceptable date".into()));
        }

        let price_region = match price_region {
            PriceRegion::NO1 => "NO1",
            PriceRegion::NO2 => "NO2",
            PriceRegion::NO3 => "NO3",
            PriceRegion::NO4 => "NO4",
            PriceRegion::NO5 => "NO5",
        };

        let year = date.year();
        let month = date.month();
        let day = date.day();
        let endpoint = format!("{}/{:02}-{:02}_{}.json", year, month, day, price_region);
        let url = self.base_url.join(endpoint.as_str())?;

        let response = self.client.get(url).send().await?;
        if response.status().is_client_error() {
            return Err(Error::Generic("Prices are not available for this date".to_string()));
        }

        Ok(response.json::<Vec<HourlyPrice>>().await?)
    }

    fn date_after_min_date(&self, given_date: &impl Datelike) -> bool {
        let year = given_date.year();
        let month = given_date.month();
        let day = given_date.day();
        let given_datetime = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        given_datetime >= MIN_DATE.unwrap()
    }
}

impl Default for Strompris {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;
    use crate::blocking;
    use crate::models::Date;

    #[tokio::test]
    async fn async_works() {
        let client = Strompris::new();

        // Just tests if the request goes through and deserializes correctly
        let date = Date::from_ymd_opt(2024, 7, 15).unwrap();
        client.get_prices(date, PriceRegion::NO1).await.unwrap();
    }

    #[test]
    fn blocking_works() {
        let date = Date::from_ymd_opt(2024, 7, 14).unwrap();
        let client = blocking::Strompris::default();
        client.get_prices(date, PriceRegion::NO1).unwrap();
    }

    #[test]
    fn blocking_works_with_chrono_date() {
        let date = NaiveDate::from_ymd_opt(2024, 7, 14).unwrap();
        let client = blocking::Strompris::default();
        client.get_prices(date, PriceRegion::NO1).unwrap();
        let date: DateTime<Utc> = DateTime::default()
            .with_year(2024)
            .unwrap()
            .with_month(7)
            .unwrap()
            .with_day(15)
            .unwrap();
        client.get_prices(date, PriceRegion::NO1).unwrap();
    }

    #[tokio::test]
    async fn async_works_with_chrono_date() {
        let date = NaiveDate::from_ymd_opt(2024, 7, 14).unwrap();
        let client = Strompris::default();
        client.get_prices(date, PriceRegion::NO1).await.unwrap();
        let date: DateTime<Utc> = DateTime::default()
            .with_year(2024)
            .unwrap()
            .with_month(7)
            .unwrap()
            .with_day(15)
            .unwrap();
        client.get_prices(date, PriceRegion::NO1).await.unwrap();
    }

    #[tokio::test]
    async fn async_returns_error_when_given_an_early_date() {
        let date = NaiveDate::from_ymd_opt(2021, 11, 30).unwrap();
        let client = Strompris::default();
        let result = client.get_prices(date, PriceRegion::NO1).await;

        assert_eq!(
            result.err().map(|e| e.to_string()).unwrap(),
            "Date is before the minimum acceptable date".to_string()
        );
    }
    #[test]
    fn blocking_returns_error_when_given_an_early_date() {
        let date = NaiveDate::from_ymd_opt(2021, 11, 30).unwrap();
        let client = blocking::Strompris::default();
        let result = client.get_prices(date, PriceRegion::NO1);
        assert_eq!(
            result.err().map(|e| e.to_string()).unwrap(),
            "Date is before the minimum acceptable date".to_string()
        );
    }

    #[test]
    fn blocking_returns_error_when_getting_price_from_futre() {
        let date = NaiveDate::from_ymd_opt(2999, 11, 30).unwrap();
        let client = blocking::Strompris::default();
        let result = client.get_prices(date, PriceRegion::NO1);
        assert_eq!(
            result.err().map(|e| e.to_string()).unwrap(),
            "Prices are not available for this date".to_string()
        );
    }

    #[tokio::test]
    async fn async_returns_error_when_getting_price_from_futre() {
        let date = NaiveDate::from_ymd_opt(2999, 11, 30).unwrap();
        let client = Strompris::default();
        let result = client.get_prices(date, PriceRegion::NO1).await;
        assert_eq!(
            result.err().map(|e| e.to_string()).unwrap(),
            "Prices are not available for this date".to_string()
        );
    }
}
