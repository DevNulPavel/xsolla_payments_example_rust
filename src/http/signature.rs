/*use eyre::{ContextCompat, WrapErr};
use sha1::Digest;
use tracing::{debug, instrument};

#[instrument(err, skip(json_data, secret_key))]
pub fn calculate_signature(
    secret_key: &str,
    json_data: &serde_json::Value,
) -> Result<String, eyre::Error> {
    unimplemented!()

    // // TODO: Может быть проще в цикле просто вызывать update для каждого значения?
    // let mut sha = sha1::Sha1::new();
    // sha.update(joined_string);
    // let result = format!("{:x}", sha.finalize());
    // debug!("Result SHA-1 hash: {}", result);

    // Ok(result)
}*/
