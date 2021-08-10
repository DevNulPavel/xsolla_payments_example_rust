use super::{checkout::Checkout, subscription::Subscription, virtual_currency::VirtualCurrency};
use serde::Deserialize;

/// Информация о покупке
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_purchase
#[derive(Debug, Deserialize)]
pub struct PurchaseInfo {
    pub virtual_currency: Option<VirtualCurrency>,
    pub checkout: Option<Checkout>,
    pub subscription: Option<Subscription>,
    pub merchant_id: i32,
}
