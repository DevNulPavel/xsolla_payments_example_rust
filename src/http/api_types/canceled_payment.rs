use super::{project_settings::ProjectSettings, purchase::PurchaseInfo};
use serde::{de::Error, Deserialize, Deserializer};


///
///
#[derive(Debug, Deserialize)]
pub struct CanceledPaymentData {
    pub settings: ProjectSettings,
    pub purchase: PurchaseInfo,
}
