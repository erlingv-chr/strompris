# Strømpris

This crate is a wrapper around the Strømpris-API hosted on https://hvakostersttrommen.no.

The crate offers both async and blocking versions of the client, as well as a strongly typed model of the 
response objects.

## Documentation
https://docs.rs/strompris

## Usage
Run `cargo add strompris` to add this crate to your `Cargo.toml` file.

### Example
```rust
use strompris::blocking::Strompris;
use strompris::{PriceRegion, Date};

fn main() {
    let date = Date::from_ymd_opt(2024, 1, 31).unwrap();
    let client = Strompris::default();
    let prices = client.get_price(date, PriceRegion::NO1).unwrap();
    for price in prices.iter() {
        println!("From: {}", price.time_start.time());
        println!("To: {}", price.time_end.time());
        println!("Price: {:.2} NOK\n", price.nok_per_kwh);
    }
}
```
