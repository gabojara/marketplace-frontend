use uuid::Uuid;

use marketplace_domain::{Contribution, Project};

mod application_repository;
mod application_service;
mod contact_information_repository;
mod project_repository;

use marketplace_domain::*;

use super::Client;

fn init_project(client: &Client) -> Project {
	let project = Project {
		id: Uuid::new_v4().to_string(),
		name: Uuid::new_v4().to_string(),
		owner: Uuid::new_v4().to_string(),
	};
	<Client as ProjectRepository>::store(client, project.clone()).unwrap();

	project
}

fn init_contribution(client: &Client) -> Contribution {
	init_contribution_with_status(client, Default::default())
}

fn init_contribution_with_status(client: &Client, status: ContributionStatus) -> Contribution {
	let project = init_project(client);

	let contribution = Contribution {
		id: Uuid::new_v4().into(),
		onchain_id: Uuid::new_v4().to_string(),
		project_id: project.id,
		status,
		..Default::default()
	};
	<Client as ContributionRepository>::create(client, contribution.clone(), Default::default())
		.unwrap();

	contribution
}
