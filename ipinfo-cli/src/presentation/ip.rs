use super::command::Command;
use anyhow::Error;
use async_trait::async_trait;
use ipinfo_api::{self, Datasource};
use reqwest;
use std::net;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[async_trait]
impl Command for Opt {
    async fn run(&self, http_client: &reqwest::Client) -> Result<(), Error> {
        match &self.subcommand {
            Subcommand::List(opt) => {
                let datasource = ipinfo_api::IpInfoApi::new(http_client);

                println!("{:#?}", datasource.list_ip(&opt.ip).await?)
            }
        };

        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    List(List),
}

#[derive(Debug, StructOpt)]
pub struct List {
    ip: Option<net::IpAddr>,
}
