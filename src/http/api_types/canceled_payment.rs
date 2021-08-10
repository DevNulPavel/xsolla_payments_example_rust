use super::{
    checkout::Checkout, project_settings::ProjectSettings, purchase::PurchaseInfo,
    subscription::Subscription, total::TotalInfo, transaction::TransactionInfo, user::UserInfo,
    refund_details::RefundDetails, virtual_items::VirtualItems
};
use serde::Deserialize;

///
///
#[derive(Debug, Deserialize)]
pub struct CanceledPaymentData {
    pub settings: ProjectSettings,
    pub purchase: PurchaseInfo,
    pub checkout: Checkout,
    pub subscription: Subscription,
    pub virtual_items: VirtualItems,
    // pub pin_codes: // TODO:
    pub total: TotalInfo,
    pub user: UserInfo,
    pub transaction: TransactionInfo,
    pub refund_details: RefundDetails,
    pub custom_parameters: Option<serde_json::Value>
}
