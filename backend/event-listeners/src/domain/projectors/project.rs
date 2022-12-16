use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use domain::{Event, GithubRepositoryId, ProjectEvent, SubscriberCallbackError};
use infrastructure::database::MappingRepository;

use crate::{
	domain::{projections::Project, EventListener, GithubService},
	infrastructure::database::{
		GithubRepoDetailsRepository, ProjectLeadRepository, ProjectRepository,
		UpdateGitubRepoIdChangeset,
	},
};

pub struct Projector {
	project_repository: ProjectRepository,
	project_lead_repository: ProjectLeadRepository,
	github_service: Arc<dyn GithubService>,
	github_repo_details_repository: GithubRepoDetailsRepository,
}

impl Projector {
	pub fn new(
		project_repository: ProjectRepository,
		project_lead_repository: ProjectLeadRepository,
		github_service: Arc<dyn GithubService>,
		github_repo_details_repository: GithubRepoDetailsRepository,
	) -> Self {
		Self {
			project_repository,
			project_lead_repository,
			github_service,
			github_repo_details_repository,
		}
	}

	async fn project_github_data(&self, github_repo_id: &GithubRepositoryId) -> Result<()> {
		let repo = self.github_service.fetch_repository_details(github_repo_id).await?;
		self.github_repo_details_repository.upsert(&repo)?;
		Ok(())
	}
}

#[async_trait]
impl EventListener for Projector {
	async fn on_event(&self, event: &Event) -> Result<(), SubscriberCallbackError> {
		if let Event::Project(event) = event {
			match event {
				ProjectEvent::Created {
					id,
					name,
					github_repo_id,
				} => {
					self.project_repository.insert(&Project::new(
						*id,
						name.to_owned(),
						(*github_repo_id).into(),
					))?;
					self.project_github_data(github_repo_id).await?;
				},
				ProjectEvent::LeaderAssigned { id, leader_id } =>
					self.project_lead_repository.insert(id, leader_id)?,
				ProjectEvent::GithubRepositoryUpdated { id, github_repo_id } => {
					self.project_repository
						.update(id, UpdateGitubRepoIdChangeset::new(*github_repo_id))?;
					self.project_github_data(github_repo_id).await?;
				},
			}
		}
		Ok(())
	}
}
