use crate::application;
use domain::{
	AggregateRootRepository, Event, PaymentRequest, Project, Publisher, UniqueMessage,
	UuidGenerator,
};
use std::sync::Arc;

pub struct Context {
	pub create_payment_request_usecase: application::payment_request::create::Usecase,
	pub create_payment_usecase: application::payment::create::Usecase,
	pub create_project_usecase: application::project::create::Usecase,
	pub assign_project_lead_usecase: application::project::assign_leader::Usecase,
}

impl Context {
	pub fn new(
		uuid_generator: Arc<dyn UuidGenerator>,
		event_publisher: Arc<dyn Publisher<UniqueMessage<Event>>>,
		project_repository: AggregateRootRepository<Project>,
		payment_request_repository: AggregateRootRepository<PaymentRequest>,
	) -> Self {
		Self {
			create_payment_request_usecase: application::payment_request::create::Usecase::new(
				uuid_generator.to_owned(),
				event_publisher.to_owned(),
				project_repository.to_owned(),
			),
			create_payment_usecase: application::payment::create::Usecase::new(
				uuid_generator.to_owned(),
				event_publisher.to_owned(),
				payment_request_repository,
			),
			create_project_usecase: application::project::create::Usecase::new(
				uuid_generator.to_owned(),
				event_publisher.to_owned(),
			),
			assign_project_lead_usecase: application::project::assign_leader::Usecase::new(
				event_publisher.to_owned(),
				project_repository,
			),
		}
	}
}

impl juniper::Context for Context {}
