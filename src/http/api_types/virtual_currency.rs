use serde::Deserialize;
use super::helpers::deserealize_currency;

/// Информация о валюте
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_virtual_currency
#[derive(Debug, Deserialize)]
pub struct VirtualCurrency {
    pub name: String,
    pub sku: String,
    pub quantity: String,
    #[serde(deserialize_with = "deserealize_currency")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}
