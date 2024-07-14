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