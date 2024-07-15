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
//! use strompris::{Strompris, PriceRegion, Date};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), reqwest::Error> {
//!     let date = Date::from_ymd_opt(2024, 1, 31).unwrap();
//!     let client = Strompris::default();
//!     let resp = client.get_price(date, PriceRegion::NO1).await?;
//!     for r in resp.iter() {
//!         dbg!(r);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! [`www.hvakosterstrommen.no`]: www.hvakosterstrommen.no

#![deny(missing_docs)]

use reqwest::Client;
use reqwest::header::HeaderMap;
use url::Url;
pub use models::HourlyPrice;
pub use models::PriceRegion;
pub use models::Date;
use chrono::Datelike;
mod models;
mod local_time_deserializer;
pub mod blocking;

/// The client for communicating with the Strømpris API hosted on
/// [`www.hvakosterstrommen.no`].
///
/// It exposes a single method for communicating with the API.
///
/// Example:
/// ```rust
/// # use strompris::{PriceRegion, Strompris, Date};
/// # #[tokio::main]
/// # async fn main() -> Result<(), reqwest::Error> {
/// let date = Date::from_ymd_opt(2024, 7, 14).unwrap();
/// let client = Strompris::default();
/// let resp = client.get_price(date, PriceRegion::NO1).await?;
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

    /// Get the price for the given date and price region.
    pub async fn get_price(
        &self,
        date: impl Datelike,
        price_region: PriceRegion,
    ) -> Result<Vec<HourlyPrice>, reqwest::Error> {

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
        let url = self.base_url.join(endpoint.as_str()).unwrap();
        self
            .client
            .get(url.as_str())
            .send()
            .await?
            .json::<Vec<HourlyPrice>>()
            .await
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
    use crate::models::Date;
    use super::*;
    use crate::blocking;

    #[tokio::test]
    async fn async_works() {
        let client = Strompris::new();

        // Just tests if the request goes through and deserializes correctly
        let date = Date::from_ymd_opt(2024, 7, 15).unwrap();
        client.get_price(date, PriceRegion::NO1).await.unwrap();
    }

    fn blocking_works() {

        let date = Date::from_ymd_opt(2024, 7, 14).unwrap();
        let client = blocking::Strompris::default();
        client.get_price(date, PriceRegion::NO1).unwrap();
    }

    fn blocking_works_with_chrono_date() {
        let date = NaiveDate::from_ymd_opt(2024, 7, 14).unwrap();
        let client = blocking::Strompris::default();
        client.get_price(date, PriceRegion::NO1).unwrap();
        let date: DateTime<Utc> = DateTime::default()
            .with_year(2024).unwrap()
            .with_month(7).unwrap()
            .with_day(15).unwrap();
        client.get_price(date, PriceRegion::NO1).unwrap();
    }

    #[tokio::test]
    async fn async_works_with_chrono_date() {
        let date = NaiveDate::from_ymd_opt(2024, 7, 14).unwrap();
        let client = Strompris::default();
        client.get_price(date, PriceRegion::NO1).await.unwrap();
        let date: DateTime<Utc> = DateTime::default()
            .with_year(2024).unwrap()
            .with_month(7).unwrap()
            .with_day(15).unwrap();
        client.get_price(date, PriceRegion::NO1).await.unwrap();
    }
}