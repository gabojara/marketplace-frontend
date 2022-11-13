use crate::{domain::EventStore, Event, EventOrigin};
use chrono::Utc;
use diesel::{query_dsl::select_dsl::SelectDsl, RunQueryDsl};
use marketplace_domain::{
	Contribution, ContributionEvent, ContributionId, Event as DomainEvent,
	EventStore as DomainEventStore, HexPrefixedString,
};
use marketplace_infrastructure::database::{schema::events, Client};
use marketplace_tests::init_pool;
use rstest::{fixture, rstest};
use serde_json::{json, Value};
use std::str::FromStr;
use uuid::Uuid;

#[fixture]
#[once]
fn database() -> Client {
	Client::new(init_pool())
}

#[fixture]
fn contribution_id() -> ContributionId {
	HexPrefixedString::from_str("0x123").unwrap().into()
}

#[fixture]
fn contributor_id() -> Uuid {
	Uuid::from_str("3d863031-e9bb-42dc-becd-67999675fb8b").unwrap()
}

#[fixture]
fn creation_event(contribution_id: ContributionId) -> Event {
	Event {
		event: ContributionEvent::Created {
			id: contribution_id,
			project_id: Default::default(),
			issue_number: Default::default(),
			gate: Default::default(),
		}
		.into(),
		timestamp: Utc::now().naive_utc(),
		deduplication_id: "dedup1".to_string(),
		origin: EventOrigin::Starknet,
		metadata: json!({"key": "value"}),
	}
}

#[fixture]
fn assigned_event(contribution_id: ContributionId, contributor_id: Uuid) -> Event {
	Event {
		event: ContributionEvent::Assigned {
			id: contribution_id,
			contributor_id,
		}
		.into(),
		timestamp: Utc::now().naive_utc(),
		deduplication_id: "dedup2".to_string(),
		origin: EventOrigin::Starknet,
		metadata: Default::default(),
	}
}

#[rstest]
#[cfg_attr(
	not(feature = "with_infrastructure_tests"),
	ignore = "infrastructure test"
)]
fn test_append_and_list(
	database: &Client,
	contribution_id: ContributionId,
	creation_event: Event,
	assigned_event: Event,
) {
	assert!(
		EventStore::append(
			database,
			&contribution_id.to_string(),
			vec![creation_event.clone(), assigned_event.clone()]
		)
		.is_ok()
	);

	let contribution_events =
		DomainEventStore::<Contribution>::list_by_id(database, &contribution_id).unwrap();
	assert_eq!(contribution_events.len(), 2);
	assert_eq!(
		DomainEvent::Contribution(contribution_events.first().unwrap().clone()),
		creation_event.event
	);
	assert_eq!(
		DomainEvent::Contribution(contribution_events.last().unwrap().clone()),
		assigned_event.event
	);
}

#[rstest]
#[cfg_attr(
	not(feature = "with_infrastructure_tests"),
	ignore = "infrastructure test"
)]
fn test_cannot_append_duplicate_event_in_same_batch(
	database: &Client,
	contribution_id: ContributionId,
	creation_event: Event,
) {
	assert!(
		EventStore::append(
			database,
			&contribution_id.to_string(),
			vec![creation_event.clone(), creation_event]
		)
		.is_err()
	);

	let contribution_events =
		DomainEventStore::<Contribution>::list_by_id(database, &contribution_id).unwrap();
	assert_eq!(contribution_events.len(), 0);
}

#[rstest]
#[cfg_attr(
	not(feature = "with_infrastructure_tests"),
	ignore = "infrastructure test"
)]
fn test_cannot_append_duplicate_event_in_different_batches(
	database: &Client,
	contribution_id: ContributionId,
	creation_event: Event,
) {
	assert!(
		EventStore::append(
			database,
			&contribution_id.to_string(),
			vec![creation_event.clone()]
		)
		.is_ok()
	);
	assert!(
		EventStore::append(database, &contribution_id.to_string(), vec![creation_event]).is_err()
	);
}

#[rstest]
#[cfg_attr(
	not(feature = "with_infrastructure_tests"),
	ignore = "infrastructure test"
)]
fn test_metadata_are_stored(
	database: &Client,
	contribution_id: ContributionId,
	creation_event: Event,
) {
	assert!(
		EventStore::append(database, &contribution_id.to_string(), vec![creation_event]).is_ok()
	);

	let connection = database.connection().unwrap();
	let list: Vec<(String, Option<Value>)> = events::dsl::events
		.select((events::dsl::origin, events::dsl::metadata))
		.load(&*connection)
		.unwrap();
	assert!(list.contains(&("starknet".to_string(), Some(json!({"key": "value"})))));
}
