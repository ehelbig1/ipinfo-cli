use anyhow::Error;
use log;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    // #[structopt(short, long)]

    // Uncomment if an API Key is needed
    // #[structopt(long, env = "OPENAI_API_KEY", hide_env_values = true)]
    // api_key: String,
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Program started!");

    Ok(())
}
