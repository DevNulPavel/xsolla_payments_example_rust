use super::helpers::currency_from_str;
use serde::Deserialize;

/// Информация о заказе
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_checkout
#[derive(Debug, Deserialize)]
pub struct CheckoutInfo {
    #[serde(deserialize_with = "currency_from_str")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}
