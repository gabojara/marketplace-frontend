use crate::domain::*;
use mapinto::ResultMapErrInto;
use mockall::automock;
use std::sync::Arc;

#[automock]
pub trait Usecase: Send + Sync {
	fn find_by_id(&self, contributor_id: ContributorId)
	-> Result<Option<Contributor>, DomainError>;
}

pub struct GetContributor {
	contributor_repository: Arc<dyn ContributorRepository>,
}

impl Usecase for GetContributor {
	fn find_by_id(
		&self,
		contributor_id: ContributorId,
	) -> Result<Option<Contributor>, DomainError> {
		self.contributor_repository.find(contributor_id).map_err_into()
	}
}

impl GetContributor {
	pub fn new_usecase_boxed(
		contributor_repository: Arc<dyn ContributorRepository>,
	) -> Box<dyn Usecase> {
		Box::new(Self {
			contributor_repository,
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use mockall::predicate::*;
	use thiserror::Error;

	#[derive(Debug, Error)]
	#[error("Oops")]
	struct Error;

	#[test]
	fn contributor_found() {
		let mut contributor_repository = MockContributorRepository::new();

		contributor_repository
			.expect_find()
			.with(eq(ContributorId::from(12)))
			.returning(|_| Ok(Some(Contributor::default())));

		let usecase = GetContributor::new_usecase_boxed(Arc::new(contributor_repository));

		let result = usecase.find_by_id(ContributorId::from(12));
		assert!(result.is_ok(), "{:?}", result.err().unwrap());
		assert_eq!(Contributor::default(), result.unwrap().unwrap());
	}

	#[test]
	fn contributor_not_found() {
		let mut contributor_repository = MockContributorRepository::new();

		contributor_repository.expect_find().returning(|_| Ok(None));

		let usecase = GetContributor::new_usecase_boxed(Arc::new(contributor_repository));

		let result = usecase.find_by_id(ContributorId::from(12));
		assert!(result.is_ok(), "{:?}", result.err().unwrap());
		assert!(result.unwrap().is_none());
	}

	#[test]
	fn find_error() {
		let mut contributor_repository = MockContributorRepository::new();

		contributor_repository
			.expect_find()
			.returning(|_| Err(ContributorRepositoryError::Infrastructure(Box::new(Error))));

		let usecase = GetContributor::new_usecase_boxed(Arc::new(contributor_repository));

		let result = usecase.find_by_id(ContributorId::from(12));
		assert!(result.is_err());
		assert_eq!(
			"Contributor repository error",
			result.unwrap_err().to_string()
		);
	}
}
