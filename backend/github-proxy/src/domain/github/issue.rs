#![allow(clippy::too_many_arguments)]

use chrono::{DateTime, Utc};
use derive_more::Constructor;
use juniper::{GraphQLEnum, GraphQLObject};
use url::Url;

#[derive(GraphQLEnum)]
pub enum Status {
	Open,
	Closed,
	Merged,
	Completed,
	Cancelled,
}

#[derive(GraphQLEnum)]
pub enum Type {
	Issue,
	PullRequest,
}

#[derive(Constructor, GraphQLObject)]
pub struct Issue {
	id: i32,
	number: i32,
	r#type: Type,
	title: String,
	html_url: Url,
	status: Status,
	created_at: DateTime<Utc>,
	merged_at: Option<DateTime<Utc>>,
	closed_at: Option<DateTime<Utc>>,
}
