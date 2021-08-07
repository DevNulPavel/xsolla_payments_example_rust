use serde::Deserialize;


/// Данные о пользователе
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_user
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