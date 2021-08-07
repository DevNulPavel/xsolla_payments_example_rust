use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString, TryFromInto};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Специальный шаблонный тип, чтобы можно было парсить возвращаемые ошибки в ответах.
/// А после этого - конвертировать в результаты.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum DataOrErrorResponse<D> {
    Ok(D),
    Err(ErrorResponse),
}
impl<D> DataOrErrorResponse<D> {
    pub fn into_result(self) -> Result<D, ErrorResponse> {
        self.into()
    }
}
impl<D> From<DataOrErrorResponse<D>> for Result<D, ErrorResponse> {
    fn from(resp: DataOrErrorResponse<D>) -> Result<D, ErrorResponse> {
        match resp {
            DataOrErrorResponse::Ok(ok) => Ok(ok),
            DataOrErrorResponse::Err(err) => Err(err),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Информация об ошибке XSolla
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_errors_handling
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde_as(as = "TryFromInto<u16>")]
    pub http_status_code: http::StatusCode,
    #[serde_as(as = "NoneAsEmptyString")]
    pub message: Option<String>,
    pub extended_message: Option<String>,
    pub request_id: String,
}
