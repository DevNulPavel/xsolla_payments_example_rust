use super::helpers::currency_from_str;
use serde::Deserialize;

/// Информация о валюте
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_virtual_currency
#[derive(Debug, Deserialize)]
pub struct VirtualCurrency {
    pub name: String,
    pub sku: String,
    pub quantity: String,
    #[serde(deserialize_with = "currency_from_str")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Информация о покупке
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase
#[derive(Debug, Deserialize)]
pub struct PurchaseInfo {
    pub virtual_currency: VirtualCurrency,
    pub merchant_id: i32,
}
