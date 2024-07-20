use crate::local_time_deserializer;
use chrono::{DateTime, FixedOffset};
use std::cmp::Ordering;

/// Wraps the resulting JSON-object, exposing each attribute.
///
/// The prices are fetched from [`ENTSO-E`] in euro, and
/// are converted by HvaKosterStrømmen using the lastest exchange rate from Norges Bank.
/// Hence, prices may vary slightly from official prices in NOK found at e.g. Nord Pool.
/// The prices are not including VAT.
///
/// [`ENTSO-E`]: https://transparency.entsoe.eu/
#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct HourlyPrice {
    /// The price per kWh in NOK, calculated using attribute `exr`
    #[serde(rename(deserialize = "NOK_per_kWh"))]
    pub nok_per_kwh: f64,
    /// The price per kWh in EUR
    #[serde(rename(deserialize = "EUR_per_kWh"))]
    pub eur_per_kwh: f64,
    /// The exchange rate from Norges Bank used to calculate `nok_per_kwh`
    #[serde(rename(deserialize = "EXR"))]
    pub exr: f64,
    /// The time this price is valid from
    #[serde(with = "local_time_deserializer")]
    pub time_start: DateTime<FixedOffset>,
    /// The time this price is valid until
    #[serde(with = "local_time_deserializer")]
    pub time_end: DateTime<FixedOffset>,
}

impl PartialOrd for HourlyPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.nok_per_kwh.partial_cmp(&other.nok_per_kwh)
    }
}
