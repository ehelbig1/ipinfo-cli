use anyhow::Error;
use std::env;
use std::time;
use structopt::StructOpt;

mod presentation;
use presentation::command::Command;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, env = "IPINFO_API_KEY", hide_env_values = true)]
    api_key: String,

    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    Ip(presentation::ip::Opt),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Program started!");

    let opt = Opt::from_args();

    let mut auth_header_value =
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", opt.api_key))?;
    auth_header_value.set_sensitive(true);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, auth_header_value);
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_str("application/json")?,
    );

    let http_client = reqwest::ClientBuilder::new()
        .timeout(time::Duration::from_secs(3))
        .default_headers(headers)
        .build()?;

    match opt.subcommand {
        Subcommand::Ip(opt) => opt.run(&http_client).await?,
    };

    Ok(())
}
