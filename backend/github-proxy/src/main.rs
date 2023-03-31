use std::sync::Arc;

use ::infrastructure::{
	config, github,
	tracing::{self, Tracer},
};
use anyhow::Result;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

mod presentation;

use serde::Deserialize;

use self::{github::RoundRobinClient, presentation::http};

#[derive(Deserialize, Clone)]
pub struct Config {
	http: ::presentation::http::Config,
	tracer: tracing::Config,
	github: github::Config,
}

#[rocket::main]
async fn main() -> Result<()> {
	dotenv().ok();
	let config: Config = config::load("backend/github-proxy/app.yaml")?;
	let _tracer = Tracer::init(&config.tracer, "github-proxy")?;

	let github: github::Client = RoundRobinClient::new(&config.github)?.into();

	http::serve(config, Arc::new(github)).await?;

	info!("👋 Gracefully shut down");
	Ok(())
}
