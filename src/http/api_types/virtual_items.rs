use serde::Deserialize;
use super::helpers::deserealize_currency;

/// Информация о виртуальном итеме покупки
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_virtual_items
#[derive(Debug, Deserialize)]
pub struct VirtualItem{
    pub sku: String,
    pub amount: i32
}

/// Информация о виртуальных покупаемых итемах
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase_virtual_items
#[derive(Debug, Deserialize)]
pub struct VirtualItems {
    pub items: Vec<VirtualItem>,
    #[serde(deserialize_with = "deserealize_currency")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: f32,
}
