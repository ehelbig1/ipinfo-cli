use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn run(&self, http_client: &reqwest::Client) -> Result<(), Error>;
}
