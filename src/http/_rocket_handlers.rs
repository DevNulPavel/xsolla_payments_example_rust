
/*use crate::{
    application::{AppConfig, Application},
    http::{api_types::callback_message::CallbackMessage, signature::calculate_signature},
    unwrap_ok_or_else, unwrap_some_or_else,
};
use bytes::Bytes;
use netaddr2::{Contains, Netv4Addr};
use rocket::{routes, get, post};
use serde::Deserialize;
use serde_json::json;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
use tracing::{debug, error, instrument};

// const ERROR_CODE_INVALID_USER: &str = "INVALID_USER";
const ERROR_CODE_INVALID_PARAMETER: &str = "INVALID_PARAMETER";
const ERROR_CODE_INVALID_SIGNATURE: &str = "INVALID_SIGNATURE";
// const ERROR_CODE_INCORRECT_AMOUNT: &str = "INCORRECT_AMOUNT";
// const ERROR_CODE_INCORRECT_INVOICE: &str = "INCORRECT_INVOICE";

//////////////////////////////////////////////////////////////////////////////////////////

#[get("/")]
#[instrument(skip(app))]
async fn index(app: Arc<Application>) -> Result<String, eyre::Error> {
    // TODO: Логировать ошибку через tap error? Или есть какой-то централизованный способ через tracing?
    let render_res = app.templates.render("index", &json!({}))?;

    Ok(render_res)
}

//////////////////////////////////////////////////////////////////////////////////////////

#[instrument(skip(app))]
async fn request_token(app: Arc<Application>) -> Option<String> {
    // Адрес получения токена
    // TODO: Закешировать в конфиге и не форматировать каждый раз
    let token_url = format!(
        "https://api.xsolla.com/merchant/v2/merchants/{}/token",
        app.config.merchant_id
    );
    debug!(%token_url, "Token request URL");

    // Параметры запроса токена
    // https://developers.xsolla.com/ru/pay-station-api/current/token/create-token/
    let data = json!({});

    // Выполним получение токена для открытия XSolla
    let request_result = app
        .http_client
        .post(token_url)
        .basic_auth(app.config.merchant_id, Some(&app.config.api_key))
        .json(&data)
        .send()
        .await;
    drop(data);
    let response = unwrap_ok_or_else!(request_result, |err| {
        error!(%err, "Token request error");
        return None;
    });

    // Получаем сразу статус и текст ответа если есть
    let status = response.status();

    // Проверяем код ответа
    if !status.is_success() {
        // Получим текст описания ошибки если есть
        let response_text = response.text().await.ok();
        error!(%status, ?response_text, "Token status error");
        return None;
    }

    // Из ответа получаем контент
    let response_text = unwrap_ok_or_else!(response.text().await, |err| {
        error!(%status, %err, "Token responce body receive failed");
        return None;
    });
    debug!(?response_text, "Received token content");

    // Пытаемся распарсить ответ
    #[derive(Deserialize, Debug)]
    struct TokenData {
        token: String,
    }
    let parse_res = serde_json::from_str::<TokenData>(&response_text);
    drop(response_text);
    let token_res = unwrap_ok_or_else!(parse_res, |err| {
        error!(%status, %err, "Token responce parsing failed");
        return None;
    });
    debug!(?token_res, "Parsed token");

    Some(token_res.token)
}

#[instrument(skip(app))]
async fn buy(app: Arc<Application>) -> Result<Response<Body>, Rejection> {
    debug!("Buy handler begin");

    // Запрашиваем токен
    let token = unwrap_some_or_else!(request_token(app).await, || {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    });

    // Создаем адрес для редиректа
    let redirect_uri = {
        let purchase_window_url = format!(
            "https://sandbox-secure.xsolla.com/paystation3/?access_token={}",
            token
        );
        unwrap_ok_or_else!(warp::http::Uri::from_str(&purchase_window_url), |err| {
            error!(%err, "Payment window open url create failed");
            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        })
    };

    Ok(warp::redirect::see_other(redirect_uri).into_response())
}

//////////////////////////////////////////////////////////////////////////////////////////

// Формируем error ответ для коллбека
fn callback_error_reply(
    code: impl AsRef<str>,
    message: impl AsRef<str>,
) -> Result<Response<Body>, Rejection> {
    // Выведем информацию об ошибке в виде JSON
    let data = warp::reply::json(&json!({
        "error": {
            "code": code.as_ref(),
            "message": message.as_ref()
        }
    }));
    Ok(warp::reply::with_status(data, StatusCode::BAD_REQUEST).into_response())
}

/// Проверяем валидность ip адреса исходного отправителя
fn check_remote_address_is_valid(remote_addr: SocketAddr) -> bool {
    let valid_addresses: [Netv4Addr; 3] = [
        Netv4Addr::new(
            Ipv4Addr::new(185, 30, 20, 0),
            Ipv4Addr::new(255, 255, 255, 0),
        ),
        Netv4Addr::new(
            Ipv4Addr::new(185, 30, 21, 0),
            Ipv4Addr::new(255, 255, 255, 0),
        ),
        Netv4Addr::new(
            Ipv4Addr::new(185, 30, 23, 0),
            Ipv4Addr::new(255, 255, 255, 0),
        ),
    ];
    valid_addresses
        .iter()
        .any(|test_addr| test_addr.contains(&remote_addr.ip()))
}

#[instrument(skip(app_config, remote_addr, body_bytes))]
async fn server_callback(
    app_config: Arc<AppConfig>,
    remote_addr: Option<SocketAddr>,
    authorization_header: String,
    body_bytes: Bytes,
) -> Result<Response<Body>, Rejection> {
    // Проверяем адрес от которого прилетел коллбек, но возможно, что надо будет убрать,
    // так как сервер может быть за reverse proxy
    if let Some(remote_addr) = remote_addr {
        let is_valid = check_remote_address_is_valid(remote_addr);
        if !is_valid {
            const MESSAGE: &str = "Remote address invalid";
            error!(?remote_addr, MESSAGE);
            return callback_error_reply(ERROR_CODE_INVALID_PARAMETER, MESSAGE);
        }
    } else {
        const MESSAGE: &str = "Remote address is missing";
        error!(MESSAGE);
        return callback_error_reply(ERROR_CODE_INVALID_PARAMETER, MESSAGE);
    }

    // Проверка подписи из заголовка на валидность
    if let Some(received_signature_value) = authorization_header.strip_prefix("Signature ") {
        let calculated_signature_value =
            calculate_signature(body_bytes.as_ref(), app_config.secret_key.as_bytes());

        debug!(
            %received_signature_value,
            %calculated_signature_value, "Signature comparison"
        );

        if calculated_signature_value != received_signature_value {
            const MESSAGE: &str = "Received signature invalid";
            error!(?remote_addr, MESSAGE);
            return callback_error_reply(ERROR_CODE_INVALID_SIGNATURE, MESSAGE);
        }
    } else {
        const MESSAGE: &str = "Signature does not start with valid prefix";
        error!(?remote_addr, MESSAGE);
        return callback_error_reply(ERROR_CODE_INVALID_SIGNATURE, MESSAGE);
    }

    debug!(?body_bytes, "Received callback data");

    // Парсим переданные байты из body
    let message = match serde_json::from_slice::<CallbackMessage>(body_bytes.as_ref()) {
        Ok(message) => message,
        Err(err) => {
            let err_message = format!("Body parsing failed with error: {}", err);
            error!(%err_message);
            return callback_error_reply(ERROR_CODE_INVALID_PARAMETER, err_message);
        }
    };

    debug!(?message, "Parsed message");

    // Обработка сообщения
    match message {
        CallbackMessage::UserValidation(_data) => {
            todo!("Implement validation");
        }
        CallbackMessage::SuccessPayment(_data) => {
            todo!("Implement success");
        }
        CallbackMessage::CanceledPayment(_data) => {
            todo!("Implement cancel");
        }
        CallbackMessage::Reject => {
            todo!("Implement reject");
        }
        CallbackMessage::Other => {}
    }

    Ok(warp::http::StatusCode::NO_CONTENT.into_response())

    // TODO:
    // - Проверка ip адреса от которого был коллбек
    // - Другие коды ошибок при ошибках внутри
    // - Проверка подписи в заголовке, при ошибке подписи - HTTP code 4xx and the error code INVALID_SIGNATURE
    // - Проверка, что не было двух одинаковых транзакций по ID
    // - Для успешной обработки код 204
    // - Для ошибки код 400
    // - 500й код ошибки при невозможности зачисления пока что
}

//////////////////////////////////////////////////////////////////////////////////////////

pub async fn start_server(app: Arc<Application>) -> Result<(), eyre::Error> {
    let config = rocket::Config {
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        port: 8080,
        ..Default::default()
    };
    rocket::custom(config)
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}*/