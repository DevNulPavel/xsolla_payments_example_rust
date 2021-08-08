use crate::application::{AppConfig, Application};
use bytes::Bytes;
use netaddr2::{Contains, Netv4Addr};
use serde_json::json;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tracing::instrument;
use warp::{filters, reject::Reject, Filter, Rejection, Reply};

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
async fn index(app: Arc<Application>) -> Result<impl Reply, Rejection> {
    let html = app
        .templates
        .render("index", &json!({}))
        .map_err(AppError::from)?;

    // TODO: Логировать ошибку через tap error? Или есть какой-то централизованный способ через tracing?

    Ok(warp::reply::html(html))
}

//////////////////////////////////////////////////////////////////////////////////////////

fn error_reply(text: impl AsRef<str>) -> Box<dyn Reply> {
    // Выведем информацию об ошибке в виде JSON
    let data = warp::reply::json(&json!({
        "code": warp::http::StatusCode::BAD_REQUEST.as_u16(),
        "message": text.as_ref()
    }));
    Box::new(warp::reply::with_status(
        data,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

#[instrument(skip(app_config, remote_addr, body_bytes))]
async fn server_callback(
    app_config: Arc<AppConfig>,
    remote_addr: Option<SocketAddr>,
    authorization_header: String,
    body_bytes: Bytes,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    // Проверяем адрес от которого прилетел коллбек, но возможно, что надо будет убрать
    // Так как сервер может быть за reverse
    if let Some(remote_addr) = remote_addr {
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
        let is_valid = valid_addresses
            .iter()
            .any(|test_addr| test_addr.contains(&remote_addr.ip()));
        if !is_valid {
            return Ok(error_reply("Remote address is invalid"));
        }
    } else {
        return Ok(error_reply("Remote address is missing"));
    }

    // TODO: Проверка подписи

    Ok(Box::new(warp::http::StatusCode::NO_CONTENT))

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
