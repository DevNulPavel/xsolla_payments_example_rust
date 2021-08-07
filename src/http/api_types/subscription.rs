use super::helpers::currency_from_str;
use serde::Deserialize;

/// Информация о подписке
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_subscription
#[derive(Debug, Deserialize)]
pub struct SubscriptionInfo {
    pub plan_id: String,
    pub subscription_id: String,
    pub product_id: String,
    pub tags: Vec<String>,
    pub date_create: chrono::DateTime<chrono::Utc>,
    pub date_next_charge: chrono::DateTime<chrono::Utc>,
    #[serde(deserialize_with = "currency_from_str")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}
