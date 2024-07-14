//! This crate offers a wrapper of the Strømpris API offered through www.hvakosterstrommen.no
//!
//! Something something strømpris

use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use reqwest::header::HeaderMap;
use url::Url;

/// Because the different regions of Norway has different access to power, Norway is
/// divided into 5 price regions. Each of these regions has its own hourly price.
/// - NO1: Oslo / Øst-Norge
/// - NO2: Kristiansand / Sør-Norge
/// - NO3: Trondheim / Midt-Norge
/// - NO4: Tromsø / Nord-Norge
/// - NO5: Bergen / Vest-Norge
pub enum PriceRegion {
    NO1,
    NO2,
    NO3,
    NO4,
    NO5,
}

/// The HourlyPrice struct wraps the resulting JSON-object, exposing each attribute.
/// The prices are fetched from [`ENTSO-E`] in euro, and
/// are converted by HvaKosterStrømmen using the lastest exchange rate from Norges Bank.
/// Hence, prices may vary slightly from official prices in NOK found at e.g. Nord Pool.
/// The prices are not including VAT.
///
/// [`ENTSO-E`]: https://transparency.entsoe.eu/
#[derive(Default, Debug, Clone, serde::Deserialize)]
pub struct HourlyPrice {
    #[serde(rename(deserialize = "NOK_per_kWh"))]
    pub nok_per_kwh: f64,
    #[serde(rename(deserialize = "EUR_per_kWh"))]
    pub eur_per_kwh: f64,
    #[serde(rename(deserialize = "EXR"))]
    pub exr: f64,
    #[serde(with = "local_time_deserializer")]
    pub time_start: DateTime<FixedOffset>,
    #[serde(with = "local_time_deserializer")]
    pub time_end: DateTime<FixedOffset>
}

/// The Strompris struct is the client for communicating with the Strømpris API hosted on
/// [`www.hvakosterstrommen.no`]. It exposes a single method for communicating with the API.
///
/// Example
/// ```rust
/// # #[tokio::main]
/// # async fn main() -> Result<(), reqwest::Error> {
/// use strompris::{PriceRegion, Strompris};
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

mod local_time_deserializer {
    use std::ops::Sub;

    use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%z";

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Get the timezone offset by finding the substring following "+"
        let tz_offset_start = s.find('+').unwrap() + 1;
        let tz_offset: i32 = s.get(tz_offset_start..tz_offset_start + 2).unwrap().parse().unwrap();


        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let hour = 3600;
        let tz = FixedOffset::east_opt(tz_offset * hour).unwrap();

        // Subtract the offset because parsing ignores timezone
        let offset_delta = Duration::hours(tz_offset as i64);
        Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(dt, tz).sub(offset_delta))
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