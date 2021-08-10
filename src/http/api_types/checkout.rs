use super::helpers::deserealize_currency;
use serde::Deserialize;

/// Информация о заказе
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_checkout
#[derive(Debug, Deserialize)]
pub struct Checkout {
    #[serde(deserialize_with = "deserealize_currency")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}
