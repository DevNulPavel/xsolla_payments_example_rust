use super::helpers::currency_from_str;
use serde::Deserialize;

/// Данные по всей покупке
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_total
#[derive(Debug, Deserialize)]
pub struct TotalInfo {
    #[serde(deserialize_with = "currency_from_str")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}