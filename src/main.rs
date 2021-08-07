mod application;
mod http;

use crate::{
    application::{AppConfig, Application},
    http::start_server,
};
use eyre::WrapErr;
use std::sync::Arc;
use url::Url;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////

async fn execute_app() -> Result<(), eyre::Error> {
    // Инициализируем менеджер логирования
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            tracing::Level::TRACE,
        ))
        // Логи только от текущего приложения, без библиотек
        .with(tracing_subscriber::filter::EnvFilter::new(env!(
            "CARGO_PKG_NAME"
        )))
        .with(tracing_subscriber::fmt::layer())
        // Для поддержки захватывания SpanTrace в eyre
        .with(tracing_error::ErrorLayer::default())
        .try_init()
        .wrap_err("Tracing init failed")?;

    // Подтягиваем окружение из файлика .env
    dotenv::dotenv().wrap_err("Dotenv read failed")?;

    // Шаблоны HTML
    let mut templates = handlebars::Handlebars::new();
    {
        templates
            .register_template_file("index", "templates/index.hbs")
            .wrap_err("Index template read failed")?;
    }

    // Адрес нашего сайта
    let site_url = {
        let site_url_string = std::env::var("SITE_URL").wrap_err("SITE_URL variable is missing")?;
        Url::parse(site_url_string.as_str()).wrap_err("SITE_URL is invalid url")?
    };

    // Идентификаторы продавца
    // ID
    let merchant_id = std::env::var("MERCHANT_ID")
        .wrap_err("MERCHANT_ID env variable is missing")?
        .parse::<u64>()
        .wrap_err("MERCHANT_ID must be u64")?;
    // Пароль
    let merchant_password =
        std::env::var("MERCHANT_PASSWORD").expect("MERCHANT_PASSWORD env variable is missing");

    // Приложение со всеми нужными нам менеджерами
    let app = Arc::new(Application {
        templates: Arc::new(templates),
        http_client: reqwest::Client::new(),
        config: Arc::new(AppConfig {
            site_url,
            merchant_id,
            merchant_password,
        }),
    });

    // Стартуем сервер
    start_server(app).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // Настройка color eyre для ошибок
    color_eyre::install().expect("Error subscription setup failed");

    // Запускаем наш код и обрабатываем ошибку если надо
    if let Err(err) = execute_app().await {
        // При ошибке не паникуем, а спокойно выводим сообщение и завершаем приложение с кодом ошибки
        // Это нужно для того, чтобы вывести содержимое ошибки, а не получать новый стектрейс из паники
        eprint!("Error! Failed with: {:?}", err);
        std::process::exit(1);
    }
}
