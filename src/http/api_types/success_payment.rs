use super::{
    checkout::Checkout, payment_details::PaymentDetails, project_settings::ProjectSettings,
    purchase::PurchaseInfo, subscription::Subscription, total::TotalInfo,
    transaction::TransactionInfo, user::UserInfo, virtual_items::VirtualItems,
};
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Данные об успешной покупке, прилетающие в наш серверный коллбек
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_webhooks_payment
#[derive(Debug, Deserialize)]
pub struct SuccessPaymentData {
    pub settings: ProjectSettings,
    pub purchase: PurchaseInfo,
    pub checkout: Checkout,
    pub subscription: Option<Subscription>,
    pub virtual_items: Option<VirtualItems>,
    // pub pin_codes
    // pub gift
    // pub promotions
    // pub coupon
    pub total: TotalInfo,
    pub user: UserInfo,
    pub transaction: TransactionInfo,
    pub payment_details: PaymentDetails,
    pub custom_parameters: Option<serde_json::Value>,
}
