use super::{
    canceled_payment::CanceledPaymentData, success_payment::SuccessPaymentData,
    user_exists_check::UserExistsCheckData,
};
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Enum сообщений в наш серверный коллбек
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_webhooks_webhooks_list
#[derive(Debug, Deserialize)]
#[serde(tag = "notification_type")]
pub enum CallbackMessage {
    #[serde(rename = "user_validation")]
    UserValidation(UserExistsCheckData),

    #[serde(rename = "payment")]
    SuccessPayment(Box<SuccessPaymentData>), // Box нужен из-за большого размера структуры внутри

    #[serde(rename = "refund")]
    CanceledPayment(Box<CanceledPaymentData>), // Box нужен из-за большого размера структуры внутри

    #[serde(rename = "afs_reject")]
    Reject, // TODO: !!!

    #[serde(other)]
    Other,
}
