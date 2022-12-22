use std::collections::HashMap;

use anyhow::anyhow;
use octocrab::{
	models::{repos::Content, Repository, User},
	FromResponse, Octocrab, OctocrabBuilder,
};
use serde::Deserialize;

mod error;
pub use error::Error;
mod specifications;

#[derive(Deserialize)]
pub struct Config {
	base_url: String,
	personal_access_token: String,
	headers: HashMap<String, String>,
}

pub struct Client(Octocrab);

impl Client {
	pub fn new(config: &Config) -> anyhow::Result<Self> {
		let instance = Octocrab::builder()
			.base_url(&config.base_url)?
			.personal_token(config.personal_access_token.clone())
			.add_headers(&config.headers)?
			.build()?;
		Ok(Self(instance))
	}

	pub async fn get_as<U, R>(&self, url: U) -> Result<R, Error>
	where
		U: AsRef<str>,
		R: FromResponse,
	{
		let result = self.0.get(url, None::<&()>).await?;
		Ok(result)
	}

	pub async fn get_repository_by_id(&self, id: u64) -> Result<Repository, Error> {
		self.get_as(format!("{}repositories/{id}", self.0.base_url)).await
	}

	pub async fn get_user_by_name(&self, username: &str) -> Result<User, Error> {
		self.get_as(format!("{}users/{username}", self.0.base_url)).await
	}

	pub async fn get_raw_file(&self, repo: &Repository, path: &str) -> Result<Content, Error> {
		let owner = repo
			.owner
			.as_ref()
			.ok_or_else(|| Error::Other(anyhow!("Missing owner in github repository")))?
			.login
			.clone();

		let mut contents = octocrab::instance()
			.repos(owner, &repo.name)
			.get_content()
			.path(path)
			.r#ref("HEAD")
			.send()
			.await?;

		contents
			.items
			.pop()
			.ok_or_else(|| Error::NotFound(anyhow!("Could not find {path} in repository")))
	}
}

trait AddHeaders: Sized {
	fn add_headers(self, headers: &HashMap<String, String>) -> anyhow::Result<Self>;
}

impl AddHeaders for OctocrabBuilder {
	fn add_headers(mut self, headers: &HashMap<String, String>) -> anyhow::Result<Self> {
		for (key, value) in headers {
			self = self.add_header(key.parse()?, value.clone());
		}
		Ok(self)
	}
}
