use super::{project_settings::ProjectSettings, purchase::PurchaseInfo};
use serde::{de::Error, Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString, TryFromInto};

////////////////////////////////////////////////////////////////////////////////////////////////////

fn currency_from_str<'de, D>(deserializer: D) -> Result<&'static iso4217::CurrencyCode, D::Error>
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

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Информация о заказе
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_checkout
#[derive(Debug, Deserialize)]
pub struct CheckoutInfo {
    #[serde(deserialize_with = "currency_from_str")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Данные по всей покупке
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_total
#[derive(Debug, Deserialize)]
pub struct TotalInfo {
    #[serde(deserialize_with = "currency_from_str")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Данные о пользователе
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_user
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub ip: std::net::IpAddr,
    pub phone: String,
    pub email: String,
    pub name: String,
    pub country: isocountry::CountryCode,
    pub zip: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Данные о транзакции
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_transaction
#[derive(Debug, Deserialize)]
pub struct TransactionInfo {
    pub id: i64,
    pub external_id: String,
    pub payment_date: chrono::DateTime<chrono::Utc>,
    pub payment_method: i32, // TODO: перегнать в enum,
    pub payment_method_order_id: String,
    pub dry_run: Option<i32>, // TODO: Перегонять в bool
    pub agreement: i32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Данные об успешной покупке, прилетающие в наш серверный коллбек
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_webhooks_payment
#[derive(Debug, Deserialize)]
pub struct SuccessPaymentData {
    pub settings: ProjectSettings,
    pub purchase: PurchaseInfo,
    pub checkout: CheckoutInfo,
    pub subscription: Option<SubscriptionInfo>,
    pub total: TotalInfo,
    pub transaction: TransactionInfo,
    pub payment_details: serde_json::Value, // TODO: https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_payment_details
    pub custom_parameters: serde_json::Value, // TODO:
                                            // pub gift
                                            // pub pin_codes
                                            // pub promotions
                                            // pub coupon
}
