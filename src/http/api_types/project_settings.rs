use serde::{de::Error, Deserialize, Deserializer};


/// Информация о приложении XSolla
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_settings
#[derive(Debug, Deserialize)]
pub struct ProjectSettings {
    pub project_id: i32,
    pub merchant_id: i32,
}
