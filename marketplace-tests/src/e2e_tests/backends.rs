use super::{database::migrated_database, http::BACKEND_BASE_URI};
use rstest::*;
use std::time::Duration;
use tokio::{
	task::{spawn, JoinHandle},
	time::interval,
};

#[fixture]
pub async fn marketplace_api() -> JoinHandle<()> {
	let handle = spawn(::marketplace_core::main());

	let mut timer = interval(Duration::from_millis(500));

	for _ in 0..10 {
		timer.tick().await;

		match reqwest::get(format!("{BACKEND_BASE_URI}/openapi.json")).await {
			Ok(response) if response.status() == reqwest::StatusCode::OK => return handle,
			_ => (),
		}
	}

	panic!("Timeout waiting for backend to be ready")
}

#[fixture]
pub async fn marketplace_indexer(_migrated_database: ()) -> JoinHandle<()> {
	let handle = spawn(::marketplace_indexer::main());

	// TODO: Find a better way to check the indexer is ready
	tokio::time::sleep(Duration::from_secs(1)).await;
	handle
}

#[fixture]
pub async fn marketplace_event_store() -> JoinHandle<()> {
	spawn(async { ::marketplace_event_store::main().await.unwrap() })
}

#[fixture]
pub async fn event_listeners() -> JoinHandle<()> {
	spawn(async { ::marketplace_core::event_listeners_main().await.unwrap() })
}
