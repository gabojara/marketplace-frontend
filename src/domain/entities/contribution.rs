use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;
use std::str::FromStr;
use thiserror::Error;
use url::Url;
use uuid::Uuid;
use wrappers::UuidWrapper;

use super::{ContributorId, ProjectId};

pub type ValidatorAddress = FieldElement;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Status {
	#[default]
	Open = 0,
	Assigned = 1,
	Completed = 2,
	Abandoned = 3,
}

pub type OnChainId = String;

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	JsonSchema,
	UuidWrapper,
	Default,
)]
pub struct Id(Uuid);

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Contribution {
	pub id: Id,
	pub onchain_id: OnChainId,
	pub project_id: ProjectId,
	pub contributor_id: Option<ContributorId>,
	pub title: Option<String>,
	pub description: Option<String>,
	pub status: Status,
	pub external_link: Option<Url>,
	pub gate: u8,
	pub metadata: Metadata,
	pub validator: ValidatorAddress,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Metadata {
	pub difficulty: Option<String>,
	pub technology: Option<String>,
	pub duration: Option<String>,
	pub context: Option<String>,
	pub r#type: Option<String>,
}

impl std::fmt::Display for Status {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Status::Open => write!(f, "OPEN"),
			Status::Assigned => write!(f, "ASSIGNED"),
			Status::Completed => write!(f, "COMPLETED"),
			Status::Abandoned => write!(f, "ABANDONED"),
		}
	}
}

#[derive(Debug, Error)]
#[error("Failed to parse `{0}` as Status")]
pub struct StatusParsingError(String);

impl FromStr for Status {
	type Err = StatusParsingError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"OPEN" => Ok(Status::Open),
			"ASSIGNED" => Ok(Status::Assigned),
			"COMPLETED" => Ok(Status::Completed),
			"ABANDONED" => Ok(Status::Abandoned),
			_ => Err(StatusParsingError(s.to_string())),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn contribution_status_from_string() {
		assert_eq!(Status::Open, "OPEN".parse().unwrap());
		assert_eq!(Status::Assigned, "ASSIGNED".parse().unwrap());
		assert_eq!(Status::Completed, "COMPLETED".parse().unwrap());
		assert_eq!(Status::Abandoned, "ABANDONED".parse().unwrap());

		assert!("NON_EXISTENT".parse::<Status>().is_err());
	}

	#[test]
	fn contribution_status_to_string() {
		assert_eq!("OPEN", Status::Open.to_string());
		assert_eq!("ASSIGNED", Status::Assigned.to_string());
		assert_eq!("COMPLETED", Status::Completed.to_string());
		assert_eq!("ABANDONED", Status::Abandoned.to_string());
	}

	#[test]
	fn contribution_status_serde() {
		for status in [
			Status::Open,
			Status::Assigned,
			Status::Completed,
			Status::Abandoned,
		] {
			assert_eq!(status, status.to_string().parse().unwrap());
		}
	}
}
