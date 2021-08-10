use serde::{
    Serialize,
    Serializer
};
use super::helpers::serialize_currency;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct VirtualItem{
    pub amount: i32,
    pub available_groups: Vec<String>,
    pub sku: String
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct VirtualItems{
    #[serde(serialize_with = "serialize_currency")]
    pub currency: &'static iso4217::CurrencyCode,
    pub items: Vec<VirtualItem>
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct VirtualCurrency{
    #[serde(serialize_with = "serialize_currency")]
    pub currency: &'static iso4217::CurrencyCode,
    pub quantity: i32
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct PurchaseInfo{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_items: Option<VirtualItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_currency: Option<VirtualCurrency>
    // coupon_code
    // gift
    // pin_codes
    // subscription
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum SandboxMode{
    Normal,
    Sandbox
}
impl Serialize for SandboxMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SandboxMode::Normal => {
                serializer.serialize_none()
            }
            SandboxMode::Sandbox => {
                serializer.serialize_str("sandbox")
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct Settings{
    #[serde(serialize_with = "serialize_currency")]
    pub currency: &'static iso4217::CurrencyCode,
    // pub external_id: String,
    // pub language: String, // TODO: 2 буквы в нижнем регистре, перегнать в ISO?
    pub mode: SandboxMode, // При sandbox url тоже должен быть https://sandbox-secure.xsolla.com,
    // pub payment_method: i32
    // pub payment_widget
    pub project_id: i32,
    // pub redirect_policy
    pub return_url: String // TODO: Может быть URL?
    // pub ui
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct UserId{
    pub value: String
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize)]
pub struct User{
    // pub age: u32,
    // pub attributes: serde_json::Value,
    // pub country
    // pub email
    pub id: UserId
    // pub is_legal
    // TODO: Остальные параметры тут https://developers.xsolla.com/ru/pay-station-api/current/token/create-token/
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Структура параметров для запроса токена
/// https://developers.xsolla.com/ru/pay-station-api/current/token/create-token/ 
#[derive(Debug, Serialize)]
pub struct Body{
    // pub custom_parameters: serde_json::Value,
    pub purchase: PurchaseInfo,
    pub settings: Settings,
    pub user: User
}