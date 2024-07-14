//! This crate offers a wrapper of the Strømpris API offered by HvaKosterStrømmen.
//!
//! See [`www.hvakosterstrommen.no`] for more info about the API.
//!
//! Example using tokio
//! ```rust
//! use strompris::{Strompris, PriceRegion};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), reqwest::Error> {
//!     let client = Strompris::default();
//!     let resp = client.get_price(2024, 7, 14, PriceRegion::NO1).await?;
//!     for r in resp.iter() {
//!         dbg!(r);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! [`www.hvakosterstrommen.no`]: www.hvakosterstrommen.no

use reqwest::Client;
use reqwest::header::HeaderMap;
use url::Url;
pub use models::HourlyPrice;
pub use models::PriceRegion;

mod models;
mod local_time_deserializer;

/// The client for communicating with the Strømpris API hosted on
/// [`www.hvakosterstrommen.no`].
///
/// It exposes a single method for communicating with the API.
///
/// Example
/// ```rust
/// # use strompris::{PriceRegion, Strompris};
/// # #[tokio::main]
/// # async fn main() -> Result<(), reqwest::Error> {
/// let client = Strompris::default();
/// let resp = client.get_price(2024, 7, 14, PriceRegion::NO1).await?;
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
        year: u32,
        month: u32,
        day: u32,
        price_region: PriceRegion,
    ) -> Result<Vec<HourlyPrice>, reqwest::Error> {

        let price_region = match price_region {
            PriceRegion::NO1 => "NO1",
            PriceRegion::NO2 => "NO2",
            PriceRegion::NO3 => "NO3",
            PriceRegion::NO4 => "NO4",
            PriceRegion::NO5 => "NO5",
        };

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
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let client = Strompris::new();

        // Just tests if the request goes through and deserializes correctly
        let r = client.get_price(2024, 7, 14, PriceRegion::NO1).await.unwrap();
        dbg!(&r);
        let first = r.last().unwrap();
        dbg!(&first.time_end.to_rfc3339());
    }
}