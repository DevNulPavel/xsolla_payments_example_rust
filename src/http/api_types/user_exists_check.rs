use super::{project_settings::ProjectSettings, user::UserInfo};
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Данные о проверке существования пользователя
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_webhooks_user_validation
#[derive(Debug, Deserialize)]
pub struct UserExistsCheckData {
    pub settings: ProjectSettings,
    pub user: UserInfo,
}
