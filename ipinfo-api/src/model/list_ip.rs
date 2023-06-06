use serde::Deserialize;
use std::net;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub ip: net::IpAddr,
    pub hostname: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
}
