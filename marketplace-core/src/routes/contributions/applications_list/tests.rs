use std::sync::Arc;

use super::list_applications;
use marketplace_core::dto;
use marketplace_domain::{
	Application, ApplicationId, ApplicationRepository, ApplicationRepositoryError,
	ApplicationStatus, ContributionId, ContributorId,
};
use rocket::{http::Status, local::blocking::Client, Build};
use uuid::Uuid;

const CONTRIBUTION_ID_1: &str = "0x1234";
struct EmptyDatabase;
impl ApplicationRepository for EmptyDatabase {
	fn create(&self, _application: Application) -> Result<(), ApplicationRepositoryError> {
		unimplemented!()
	}

	fn update(&self, _application: Application) -> Result<(), ApplicationRepositoryError> {
		unimplemented!()
	}

	fn find(&self, _id: &ApplicationId) -> Result<Option<Application>, ApplicationRepositoryError> {
		unimplemented!()
	}

	fn list_by_contribution(
		&self,
		_contribution_id: &ContributionId,
		_contributor_id: Option<ContributorId>,
	) -> Result<Vec<Application>, ApplicationRepositoryError> {
		Ok(vec![])
	}

	fn list_by_contributor(
		&self,
		_contributor_id: Option<ContributorId>,
	) -> Result<Vec<Application>, ApplicationRepositoryError> {
		unimplemented!()
	}
}
struct FilledDatabase;
impl ApplicationRepository for FilledDatabase {
	fn create(&self, _application: Application) -> Result<(), ApplicationRepositoryError> {
		unimplemented!()
	}

	fn update(&self, _application: Application) -> Result<(), ApplicationRepositoryError> {
		unimplemented!()
	}

	fn find(&self, _id: &ApplicationId) -> Result<Option<Application>, ApplicationRepositoryError> {
		unimplemented!()
	}

	fn list_by_contribution(
		&self,
		_contribution_id: &ContributionId,
		_contributor_id: Option<ContributorId>,
	) -> Result<Vec<Application>, ApplicationRepositoryError> {
		Ok(vec![
			Application::new(Uuid::from_u128(0).into(), 0.into(), 0u128.into()),
			Application::new(Uuid::from_u128(1).into(), 0.into(), 1u128.into()),
		])
	}

	fn list_by_contributor(
		&self,
		_contributor_id: Option<ContributorId>,
	) -> Result<Vec<Application>, ApplicationRepositoryError> {
		unimplemented!()
	}
}

fn rocket() -> rocket::Rocket<Build> {
	rocket::build().mount("/", routes![list_applications])
}

#[test]
fn ok_empty() {
	let uri = format!("/contributions/{CONTRIBUTION_ID_1}/applications");
	let client = Client::untracked(
		rocket().manage(Arc::new(EmptyDatabase) as Arc<dyn ApplicationRepository>),
	)
	.expect("valid rocket instance");
	let response = client.get(uri).dispatch();

	assert_eq!(response.status(), Status::Ok);

	assert_eq!(
		Vec::<dto::Application>::new(),
		response.into_json::<Vec<dto::Application>>().unwrap()
	);
}

#[test]
fn ok_multiple() {
	let uri = format!("/contributions/{CONTRIBUTION_ID_1}/applications");
	let client = Client::untracked(
		rocket().manage(Arc::new(FilledDatabase) as Arc<dyn ApplicationRepository>),
	)
	.expect("valid rocket instance");
	let response = client.get(uri).dispatch();

	assert_eq!(response.status(), Status::Ok);
	assert_eq!(
		vec![
			dto::Application {
				id: Uuid::from_u128(0).to_string(),
				contribution_id: String::from("0x00"),
				contributor_id: ContributorId::from(0).to_string(),
				status: ApplicationStatus::Pending.to_string(),
			},
			dto::Application {
				id: Uuid::from_u128(1).to_string(),
				contribution_id: String::from("0x00"),
				contributor_id: ContributorId::from(1).to_string(),
				status: ApplicationStatus::Pending.to_string(),
			},
		],
		response.into_json::<Vec<dto::Application>>().unwrap()
	);
}

#[test]
fn ok_specifying_contributor() {
	let uri = format!(
		"/contributions/{CONTRIBUTION_ID_1}/applications?contributor_id=0x0000000000000000000000000000000000000000000000000000000000000000"
	);
	let client = Client::untracked(
		rocket().manage(Arc::new(EmptyDatabase) as Arc<dyn ApplicationRepository>),
	)
	.expect("valid rocket instance");
	let response = client.get(uri).dispatch();

	assert_eq!(response.status(), Status::Ok);
	assert_eq!(
		Vec::<dto::Application>::new(),
		response.into_json::<Vec<dto::Application>>().unwrap()
	);
}
