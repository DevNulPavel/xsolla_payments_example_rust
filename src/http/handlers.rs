use crate::{
    application::{AppConfig, Application},
    http::{api_types::callback_message::CallbackMessage, signature::calculate_signature},
};
use bytes::Bytes;
use netaddr2::{Contains, Netv4Addr};
use serde_json::json;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tracing::{debug, error, instrument};
use warp::{
    filters,
    http::{Response, StatusCode},
    hyper::Body,
    reject::Reject,
    Filter, Rejection, Reply,
};

const ERROR_CODE_INVALID_USER: &str = "INVALID_USER";
const ERROR_CODE_INVALID_PARAMETER: &str = "INVALID_PARAMETER";
const ERROR_CODE_INVALID_SIGNATURE: &str = "INVALID_SIGNATURE";
const ERROR_CODE_INCORRECT_AMOUNT: &str = "INCORRECT_AMOUNT";
const ERROR_CODE_INCORRECT_INVOICE: &str = "INCORRECT_INVOICE";

//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct AppError(pub eyre::Error);

impl Reject for AppError {}

// impl From<eyre::Error> for AppError {
//     fn from(err: eyre::Error) -> AppError {
//         AppError(err)
//     }
// }

impl<E> From<E> for AppError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: E) -> AppError {
        AppError(eyre::Error::from(err))
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

#[instrument(skip(app))]
async fn index(app: Arc<Application>) -> Result<Response<Body>, Rejection> {
    // TODO: Логировать ошибку через tap error? Или есть какой-то централизованный способ через tracing?
    let render_res = app.templates.render("index", &json!({}));

    match render_res {
        Ok(html) => Ok(warp::reply::html(html).into_response()),
        Err(err) => {
            error!(%err, "Template render error");
            Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

#[instrument(skip(app))]
async fn buy(app: Arc<Application>) -> Result<Response<Body>, Rejection> {
    Ok(warp::redirect::see_other(warp::http::Uri::from_static("test")).into_response())
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
        CallbackMessage::UserValidation(data) => {
            todo!("Implement validation");
        }
        CallbackMessage::SuccessPayment(data) => {
            todo!("Implement success");
        }
        CallbackMessage::CanceledPayment(data) => {
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

// #[instrument]
// async fn rejection_to_json(rejection: Rejection) -> Result<impl Reply, Rejection> {
//     // Если это какая-то наша ошибка, то отдаем 400й код ошибки + описание
//     if let Some(err) = rejection.find::<eyre::Error>() {
//         let reply = warp::reply::json(&json!({
//             "code": warp::http::StatusCode::BAD_REQUEST.as_u16(),
//             "message": err.to_string()
//         }));
//         Ok(warp::reply::with_status(
//             reply,
//             warp::http::StatusCode::BAD_REQUEST,
//         ))
//     } else {
//         Err(rejection)
//     }
// }

//////////////////////////////////////////////////////////////////////////////////////////

pub async fn start_server(app: Arc<Application>) -> Result<(), eyre::Error> {
    // Маршрут для коллбека после покупки
    let purchase_server_cb = warp::path::path("server_callback")
        // Это POST запрос должен быть
        .and(warp::post())
        // Json content type
        .and(filters::header::exact_ignore_case(
            "Content-Type",
            "application/json",
        ))
        // Конфиг наш
        .and(warp::any().map({
            let config = app.config.clone();
            move || config.clone()
        }))
        // Удаленный адрес
        .and(filters::addr::remote())
        // Заголовки запроса
        .and(filters::header::header::<String>("Authorization"))
        // Тело в виде сырых байт
        .and(filters::body::bytes())
        // Обработчик
        .and_then(server_callback)
        // В случае нашей ошибки возвращаем код 400 + сообщение об ошибке
        // .recover(rejection_to_json)
        // Добавляем трассировку данного запроса
        .with(warp::trace::request());

    let routes = purchase_server_cb.with(warp::trace::request());

    warp::serve(routes).bind(([0, 0, 0, 0], 8080)).await;

    Ok(())
}
