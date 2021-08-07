use super::{
    checkout::CheckoutInfo, project_settings::ProjectSettings, purchase::PurchaseInfo,
    subscription::SubscriptionInfo, total::TotalInfo, transaction::TransactionInfo, user::UserInfo,
    refund_details::RefundDetails
};
use serde::Deserialize;

///
///
#[derive(Debug, Deserialize)]
pub struct CanceledPaymentData {
    pub settings: ProjectSettings,
    pub purchase: PurchaseInfo,
    pub checkout: CheckoutInfo,
    pub subscription: SubscriptionInfo, // TODO: Checkout
    pub total: TotalInfo,
    pub user: UserInfo,
    pub transaction: TransactionInfo,
    pub refund_details: RefundDetails,
    pub custom_parameters: Option<serde_json::Value>
    // pub virtual_items:
    // pub pin_codes
}
