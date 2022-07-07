use crate::{
    database::schema::*,
    domain::{self},
};
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

use super::projects::Project;

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone)]
#[belongs_to(Project)]
#[serde(crate = "rocket::serde")]
pub struct Contribution {
    pub id: String,
    pub project_id: String,
    pub status: String,
    pub transaction_hash: Option<String>,
    pub author: String,
    pub gate: i16,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "contributions"]
pub struct AssignContributionForm {
    pub id: String,
    pub status: String,
    pub author: String,
    pub transaction_hash: Option<String>,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "contributions"]
pub struct ValidateContributionForm {
    pub id: String,
    pub status: String,
    pub transaction_hash: Option<String>,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "contributions"]
pub struct ContributionContractUpdateForm {
    pub id: String,
    pub transaction_hash: String,
}

#[derive(Insertable)]
#[table_name = "contributions"]
pub struct NewContribution {
    pub id: String,
    pub project_id: String,
    pub status: String,
    pub author: String,
    pub gate: i16,
    pub transaction_hash: Option<String>,
}

impl From<domain::Contribution> for NewContribution {
    fn from(contribution: domain::Contribution) -> Self {
        Self {
            id: contribution.id,
            project_id: contribution.project_id,
            status: contribution.status.to_string(),
            author: contribution.author,
            gate: contribution.gate.into(),
            transaction_hash: None,
        }
    }
}

impl From<domain::ContractUpdateStatus> for ContributionContractUpdateForm {
    fn from(status: domain::ContractUpdateStatus) -> Self {
        Self {
            id: status.contribution_id,
            transaction_hash: status.transaction_hash,
        }
    }
}

impl From<Contribution> for domain::Contribution {
    fn from(contribution: Contribution) -> Self {
        Self {
            id: contribution.id,
            author: contribution.author,
            project_id: contribution.project_id,
            status: contribution.status.parse().unwrap(),
            // Safe to unwrap because the value stored can only come from an u8
            gate: contribution.gate.try_into().unwrap(),
        }
    }
}
