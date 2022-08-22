mod status;
pub use status::Status;

use marketplace_wrappers::UuidWrapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;
use url::Url;
use uuid::Uuid;

use super::{ContributorId, ProjectId};

pub type ValidatorAddress = FieldElement;

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
