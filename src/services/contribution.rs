use crate::client::contribution_contract::ContributionStarknetContractClient;
use crate::db::{
    self,
    models::*,
    schema::projects::{self, dsl::*},
};
use crate::model::github::RepositoryWithExtension;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use log::debug;

const GITHUB_API_ROOT: &str = "https://api.github.com";

pub struct RepoAnalyzer {
    pub contribution_contract_client: ContributionStarknetContractClient,
}

impl RepoAnalyzer {
    pub fn new(contribution_contract_client: ContributionStarknetContractClient) -> Self {
        Self {
            contribution_contract_client,
        }
    }

    pub async fn analyze(&self, organisation_name: &str, repository_name: &str) -> Result<()> {
        debug!(
            "Entering analyze with args: {} - {}",
            organisation_name, repository_name
        );
        let connection = db::establish_connection()?;

        let octo = octocrab::instance();
        let repo = octo
            .get::<RepositoryWithExtension, String, ()>(
                format!(
                    "{}/repos/{}/{}",
                    GITHUB_API_ROOT, organisation_name, repository_name
                ),
                None::<&()>,
            )
            .await?;
        println!("open_issues: {:?}", repo.open_issues);
        let results = projects
            .filter(organisation.eq(organisation_name))
            .filter(repository.eq(repository_name))
            .limit(1)
            .load::<Project>(&connection)
            .expect("Error loading projects");

        if results.is_empty() {
            self.create_project(&connection, organisation_name, repository_name)?;
        }

        // TODO: option 1: Octocrab PullRequestHandler.list
        // https://docs.rs/octocrab/latest/octocrab/pulls/struct.PullRequestHandler.html#method.list
        // TODO: handle pagination
        let page = octo
            .pulls(organisation_name, repository_name)
            .list()
            .state(octocrab::params::State::Closed)
            .direction(octocrab::params::Direction::Ascending)
            .per_page(255)
            .page(0u32)
            .send()
            .await?;
        for pr in page.items {
            // TODO: check if PR exists in DB
            // TODO: check if PR was merged
            // TODO: invoke smart contract to update state
            // TODO: update DB
            let is_merged = pr.merged_at.is_some();
            let pr_id = pr.id;
            let author = pr.user.unwrap().login;
            println!("PR #{} is merged: {} by: {}", pr_id, is_merged, author);
            self.contribution_contract_client
                .register_contribution(
                    organisation_name,
                    repository_name,
                    author,
                    pr_id.as_ref().to_string(),
                )
                .await
                .map_err(anyhow::Error::msg)?;
        }
        Ok(())
    }

    fn create_project(
        &self,
        conn: &PgConnection,
        organisation_name: &str,
        repository_name: &str,
    ) -> Result<Project> {
        let new_project = NewProject {
            organisation: String::from(organisation_name),
            repository: String::from(repository_name),
        };
        diesel::insert_into(projects::table)
            .values(&new_project)
            .get_result(conn)
            .map_err(|e| anyhow!(e))
    }
}
