use serde::Deserialize;
use super::{success_payment::SuccessPaymentData, canceled_payment::CanceledPaymentData};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Enum сообщений в наш серверный коллбек
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_webhooks_webhooks_list
#[derive(Debug, Deserialize)]
#[serde(tag = "notification_type")]
pub enum CallbackMessage {
    #[serde(rename = "user_validation")]
    UserValidation, // TODO: !!!

    #[serde(rename = "payment")]
    SuccessPayment(SuccessPaymentData),

    #[serde(rename = "refund")]
    CanceledPayment(CanceledPaymentData),

    #[serde(other)]
    Other
}
