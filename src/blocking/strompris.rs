use chrono::{Datelike, NaiveDate};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use url::Url;

use crate::Error;
use crate::PriceRegion;
use crate::Result;
use crate::{HourlyPrice, MIN_DATE};

/// The blocking version of [`Strompris`].
///
/// Example:
/// ```rust
/// use strompris::blocking::Strompris;
/// use strompris::{Date, PriceRegion};
///
/// let date = Date::from_ymd_opt(2024, 1, 31).unwrap();
/// let client = Strompris::default();
/// let prices = client.get_prices(date, PriceRegion::NO1).unwrap();
/// for price in prices.iter() {
///     println!("Price: {:.2}", price.nok_per_kwh);
///     println!("From: {}", price.time_start.time());
///     println!("To: {}\n", price.time_end.time());
/// }
/// ```
/// [`Strompris`]: crate::Strompris
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
    ///
    /// The prices are represented by a vector consisting of 24 hourly prices.
    ///
    /// Note: The API does not know the future! Tomorrow's prices are usually ready by 13:00,
    /// local time.
    pub fn get_prices(&self, date: impl Datelike, price_region: PriceRegion) -> Result<Vec<HourlyPrice>> {
        let price_region = match price_region {
            PriceRegion::NO1 => "NO1",
            PriceRegion::NO2 => "NO2",
            PriceRegion::NO3 => "NO3",
            PriceRegion::NO4 => "NO4",
            PriceRegion::NO5 => "NO5",
        };

        if !self.date_after_min_date(&date) {
            return Err(Error::Generic("Date is before the minimum acceptable date".to_string()));
        }

        let year = date.year();
        let month = date.month();
        let day = date.day();
        let endpoint = format!("{}/{:02}-{:02}_{}.json", year, month, day, price_region);
        let url = self.base_url.join(endpoint.as_str()).unwrap();
        let response = self.client.get(url).send()?;
        if response.status().is_client_error() {
            return Err(Error::Generic("Prices are not available for this date".to_string()));
        }

        Ok(response.json::<Vec<HourlyPrice>>()?)
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
        Strompris::new()
    }
}
