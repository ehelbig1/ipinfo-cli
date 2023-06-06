use async_trait::async_trait;
use std::net;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn list_ip(
        &self,
        ip: &Option<net::IpAddr>,
    ) -> Result<model::list_ip::Response, error::Error>;
}

pub struct IpInfoApi<'a> {
    http_client: &'a reqwest::Client,
    base_url: String,
}

impl<'a> IpInfoApi<'a> {
    pub fn new(http_client: &'a reqwest::Client) -> Self {
        Self {
            http_client,
            base_url: String::from("https://ipinfo.io"),
        }
    }
}

#[async_trait]
impl<'a> Datasource for IpInfoApi<'a> {
    async fn list_ip(
        &self,
        ip: &Option<net::IpAddr>,
    ) -> Result<model::list_ip::Response, error::Error> {
        let url = if let Some(ip) = ip {
            format!("{}/{}", self.base_url, ip)
        } else {
            self.base_url.to_string()
        };

        let response = self.http_client.get(url).send().await?.json().await?;

        Ok(response)
    }
}
