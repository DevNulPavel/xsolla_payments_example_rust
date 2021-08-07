use std::{
    sync::{
        Arc
    }
};
use handlebars::{
    Handlebars
};
use reqwest::{
    Client
};


#[derive(Debug)]
pub struct AppConfig{
    pub current_site_url: url::Url,
    pub secret_key: String
}

#[derive(Debug)]
pub struct Application{
    pub templates: Arc<Handlebars<'static>>,
    pub http_client: Client, // Arc inside
    pub config: Arc<AppConfig>
}