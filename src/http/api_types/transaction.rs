use serde::Deserialize;

/// Данные о транзакции
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_payment_transaction
#[derive(Debug, Deserialize)]
pub struct TransactionInfo {
    pub id: i64,
    pub external_id: String,
    pub payment_date: chrono::DateTime<chrono::Utc>,
    pub payment_method: i32, // TODO: перегнать в enum,
    pub payment_method_order_id: String,
    pub dry_run: Option<i32>, // TODO: Перегонять в bool
    pub agreement: i32,
}
