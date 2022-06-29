mod models;

use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use log::info;
use octocrab::{models::pulls::PullRequest, Page};
use std::sync::Arc;

use crate::{
    model::{pullrequest, repository},
    traits::{fetcher::*, Streamable},
};

impl From<models::RepositoryWithExtension> for repository::Repository {
    fn from(repo: models::RepositoryWithExtension) -> Self {
        Self {
            id: repo.inner.id.to_string(),
            owner: repo
                .inner
                .owner
                .expect("Invalid repo owner received from github API")
                .login,
            name: repo.inner.name,
        }
    }
}

pub struct API {
    octo: Arc<octocrab::Octocrab>,
}

impl API {
    pub fn new() -> Self {
        API {
            octo: octocrab::instance(),
        }
    }
}

impl Default for API {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Fetcher<pullrequest::Filter, pullrequest::PullRequest> for API {
    async fn fetch(&self, filter: pullrequest::Filter) -> FetchResult<pullrequest::PullRequest> {
        const MAX_PR_PER_PAGE: u8 = 100;

        let repository = filter.repository.expect("Repository is mandatory for now");

        info!(
            "Fetching repository {}/{}",
            repository.owner, repository.name
        );

        let filtered =
            |page: Page<PullRequest>| page.into_iter().filter(|pr| pr.merged_at.is_some());

        // List the closed PRs
        let page = self
            .octo
            .pulls(&repository.owner, &repository.name)
            .list()
            .state(octocrab::params::State::Closed)
            .direction(octocrab::params::Direction::Ascending)
            .per_page(MAX_PR_PER_PAGE)
            .send()
            .await?;

        let stream = stream::unfold(
            (self.octo.clone(), page.next.clone(), filtered(page)),
            move |(octo, next_page, mut iter)| async move {
                if let Some(next_item) = iter.next() {
                    return Some((next_item, (octo, next_page, iter)));
                }

                if let Ok(Some(page)) = octo.get_page(&next_page).await {
                    let next_page = page.next.clone();
                    let mut iter = filtered(page);

                    if let Some(next_item) = iter.next() {
                        return Some((next_item, (octo, next_page, iter)));
                    }
                }

                None
            },
        );

        let result = stream.map(|pr| pr.into());

        Ok(Streamable::Async(result.into()))
    }
}

#[async_trait]
impl Fetcher<repository::Filter, repository::Repository> for API {
    async fn fetch(&self, filter: repository::Filter) -> FetchResult<repository::Repository> {
        info!("Fetching repository with filter {:?}", filter);

        const GITHUB_API_ROOT: &str = "https://api.github.com";

        let repo = self
            .octo
            .get::<models::RepositoryWithExtension, String, ()>(
                format!(
                    "{}/repos/{}/{}",
                    GITHUB_API_ROOT,
                    filter.owner.expect("Repository owner is mandatory"),
                    filter.name.expect("Repository name is mandatory")
                ),
                None::<&()>,
            )
            .await?;

        Ok(Streamable::Async(
            stream::once(async { repo.into() }).into(),
        ))
    }
}
