use serde::Deserialize;
use super::helpers::currency_from_str;

/// Инфа платежа
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_refund_payment_details_payment
#[derive(Debug, Deserialize)]
pub struct Payment {
    #[serde(deserialize_with = "currency_from_str")]
    pub currency: &'static iso4217::CurrencyCode,
    pub amount: String
}

/// Детали платежа
/// https://developers.xsolla.com/ru/api/v2/getting-started/#api_param_webhooks_refund_payment_details
#[derive(Debug, Deserialize)]
pub struct PaymentDetails {
    payment: Payment,
    payment_method_sum: Payment,
    xsolla_balance_sum: Payment,
    payout: Payment,
    vat: Option<Payment>,
    xsolla_fee: Payment,
    payment_method_fee: Payment,
    sales_tax: Payment,
    direct_wht: Payment,
    repatriation_commission: Payment,
    payout_currency_rate: f32
}
