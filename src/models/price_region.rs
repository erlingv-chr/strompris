use derive_more::Display;

/// Because the different regions of Norway has different access to power, Norway is
/// divided into 5 price regions. Each of these regions has its own hourly price.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display)]
pub enum PriceRegion {
    /// Oslo / Øst-Norge
    NO1,
    /// Kristiansand / Sør-Norge
    NO2,
    /// Trondheim / Midt-Norge
    NO3,
    /// Tromsø / Nord-Norge
    NO4,
    /// Bergen / Vest-Norge
    NO5,
}
