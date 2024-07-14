use chrono::{DateTime, FixedOffset};
use crate::local_time_deserializer;

/// Wraps the resulting JSON-object, exposing each attribute.
///
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