use super::Database;

use crate::database::schema::projects::dsl::*;
use crate::domain::entities::{contribution::Contribution, project::Project};
use crate::domain::repositories::project::ProjectRepository;
use crate::domain::value_objects::ProjectWithContributions;
use crate::{database::models as db_model, domain::errors::Error};
use diesel::prelude::*;
use diesel::query_dsl::BelongingToDsl;
use itertools::Itertools;

impl ProjectRepository for Database {
    fn find_all_with_contributions(&self) -> Result<Vec<ProjectWithContributions>, Error> {
        let project_list = projects
            .load::<db_model::Project>(self.connection())
            .map_err(|_| Error)?;

        let contribution_list = db_model::Contribution::belonging_to(&project_list)
            .load::<db_model::Contribution>(self.connection())
            .map_err(|_| Error)?
            .grouped_by(&project_list);

        let result = project_list
            .into_iter()
            .zip(contribution_list)
            .map(ProjectWithContributions::from);

        Ok(result)
    }
}

impl From<(db_model::Project, Vec<db_model::Contribution>)> for ProjectWithContributions {
    fn from((project, contributions): (db_model::Project, Vec<db_model::Contribution>)) -> Self {
        Self {
            project: project.into(),
            contributions: contributions.into_iter().map_into().collect(),
        }
    }
}

impl From<db_model::Project> for Project {
    fn from(project: db_model::Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
            owner: project.owner,
        }
    }
}

impl From<db_model::Contribution> for Contribution {
    fn from(contribution: db_model::Contribution) -> Self {
        Self {
            id: contribution.id,
            contributor_id: {
                if contribution.contributor_id.is_empty() {
                    None
                } else {
                    Some(contribution.contributor_id.into())
                }
            },
            project_id: contribution.project_id,
            status: contribution
                .status
                .parse()
                .unwrap_or(ContributionStatus::Open),
            // Safe to unwrap because the value stored can only come from an u8
            gate: contribution.gate.try_into().unwrap(),
            description: contribution.description,
            external_link: contribution
                .external_link
                .map(|link| url::Url::parse(&link).unwrap()),
            title: contribution.title,
            metadata: domain::ContributionMetadata {
                difficulty: contribution.difficulty,
                technology: contribution.technology,
                duration: contribution.duration,
                context: contribution.context,
                r#type: contribution.type_,
            },
        }
    }
}
