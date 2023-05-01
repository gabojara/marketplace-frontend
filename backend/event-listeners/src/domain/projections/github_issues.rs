use ::infrastructure::database::schema::*;
use chrono::NaiveDateTime;
use derive_new::new;
use domain::{
	GithubIssueId, GithubIssueNumber, GithubIssueStatus, GithubIssueType, GithubRepoId,
	GithubUserId,
};
use serde::{Deserialize, Serialize};

#[allow(clippy::too_many_arguments)]
#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize, new)]
pub struct GithubIssue {
	id: GithubIssueId,
	repo_id: GithubRepoId,
	issue_number: GithubIssueNumber,
	created_at: NaiveDateTime,
	author_id: GithubUserId,
	merged_at: Option<NaiveDateTime>,
	type_: GithubIssueType,
	status: GithubIssueStatus,
	title: String,
	html_url: String,
	closed_at: Option<NaiveDateTime>,
}

impl domain::Entity for GithubIssue {
	type Id = GithubIssueId;
}
