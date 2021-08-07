use super::{
    checkout::CheckoutInfo, project_settings::ProjectSettings, purchase::PurchaseInfo,
    subscription::SubscriptionInfo, total::TotalInfo, transaction::TransactionInfo, user::UserInfo,
    payment_details::PaymentDetails
};
use serde::Deserialize;

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
    pub user: UserInfo,
    pub transaction: TransactionInfo,
    pub payment_details: PaymentDetails,
    pub custom_parameters: Option<serde_json::Value>, 
    // pub gift
    // pub pin_codes
    // pub promotions
    // pub coupon
}
