use super::{project_settings::ProjectSettings, purchase::PurchaseInfo};
use serde::{de::Error, Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString, TryFromInto};

///
///
#[derive(Debug, Deserialize)]
pub struct CanceledPaymentData {
    pub settings: ProjectSettings,
    pub purchase: PurchaseInfo,
}
