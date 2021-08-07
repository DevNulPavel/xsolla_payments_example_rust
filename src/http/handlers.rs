use crate::application::{AppConfig, Application};
use bytes::Bytes;
use serde_json::json;
use std::{sync::Arc, net::SocketAddr};
use tracing::instrument;
use warp::{reject::Reject, filters, Filter, Rejection, Reply};

//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct AppError(pub eyre::Error);

impl Reject for AppError {}

impl<E> From<E> for AppError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: E) -> AppError {
        AppError(eyre::Error::new(err))
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

#[instrument(skip(app_config, remote_addr, body_bytes))]
async fn purchase_server_callback(
    app_config: Arc<AppConfig>,
    remote_addr: Option<SocketAddr>,
    authorization_header: String,
    body_bytes: Bytes,
) -> Result<impl Reply, Rejection> {
    // TODO:
    // - Проверка ip адреса от которого был коллбек
    // - Другие коды ошибок при ошибках внутри
    // - Проверка подписи в заголовке
    // - Проверка, что не было двух одинаковых транзакций по ID
    // - Для успешной обработки код 204
    // - Для ошибки код 400
    // - 500й код ошибки при невозможности зачисления пока что

    Ok(http::StatusCode::NO_CONTENT)
}

//////////////////////////////////////////////////////////////////////////////////////////

#[instrument]
async fn rejection_to_json(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = rejection.find::<eyre::Error>(){
        let reply = warp::reply::json(&json!({
            "code": warp::http::StatusCode::BAD_REQUEST.as_u16(),
            "message": err.to_string()
        }));
        Ok(warp::reply::with_status(reply, warp::http::StatusCode::BAD_REQUEST))
    }else{
        Err(rejection)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

pub async fn start_server(app: Arc<Application>) -> Result<(), eyre::Error> {
    // Маршрут для коллбека после покупки
    let purchase_server_cb = warp::path::path("purchase_server_callback_url")
        // Это POST запрос должен быть
        .and(warp::post())
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
        .and_then(purchase_server_callback)
        // В случае нашей ошибки возвращаем код 400 + сообщение об ошибке
        .recover(rejection_to_json);

    let routes = purchase_server_cb.with(warp::trace::request());

    warp::serve(routes).bind(([0, 0, 0, 0], 8080)).await;

    Ok(())
}
