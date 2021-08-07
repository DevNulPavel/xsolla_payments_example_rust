use serde::Deserialize;

/// Информация о возврате
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_refund_refund_details
#[derive(Debug, Deserialize)]
pub struct RefundDetails {
    pub code: i32,
    pub reason: String,
    pub author: String
}
