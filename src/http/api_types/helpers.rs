
use serde::{de::Error, Deserialize, Deserializer};

pub fn currency_from_str<'de, D>(deserializer: D) -> Result<&'static iso4217::CurrencyCode, D::Error>
where
    D: Deserializer<'de>,
{
    let text: &str = Deserialize::deserialize(deserializer)?;
    if text.len() != 3 {
        return Err(D::Error::custom("Must be 3 symbols for currency parsing"));
    }
    let code = iso4217::alpha3(text).ok_or_else(|| D::Error::custom("Invalid currency code"))?;
    Ok(code)
}
