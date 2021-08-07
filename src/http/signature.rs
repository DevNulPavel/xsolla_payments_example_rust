use sha1::Digest;
use tracing::instrument;

#[instrument(skip(json_data, secret_key))]
pub fn calculate_signature(json_data: &[u8], secret_key: &[u8]) -> String {
    let mut sha = sha1::Sha1::new();

    // TODO: Нужен ли вектор? Или достаточно делать update несколько раз?
    // let mut buffer = Vec::with_capacity(json_data.len() + secret_key.len());
    // buffer.extend_from_slice(json_data);
    // buffer.extend_from_slice(secret_key);
    // sha.update(buffer);

    // TODO: Нужен ли вектор? Или достаточно делать update несколько раз?
    sha.update(json_data);
    sha.update(secret_key);

    let result = format!("{:x}", sha.finalize());

    result
}
