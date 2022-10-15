use lapin::{options::QueueDeclareOptions, Connection};
use log::info;
use marketplace_infrastructure::amqp::{Error as EventBusError, EventBus};

pub const QUEUE_NAME: &str = "event-store";

pub async fn consumer() -> Result<EventBus, EventBusError> {
	let connection = Connection::connect(&amqp_address()?, Default::default()).await?;
	info!("🔗 Event store connected");
	EventBus::new(connection)
		.await?
		.with_queue(
			QUEUE_NAME,
			QueueDeclareOptions {
				durable: true,      // persist messages during restart
				exclusive: true,    // only one consumer on this queue
				auto_delete: false, // do not delete the queue at shutdown
				..Default::default()
			},
		)
		.await
}

pub async fn publisher() -> Result<EventBus, EventBusError> {
	let connection = Connection::connect(&amqp_address()?, Default::default()).await?;
	info!("🔗 Event store connected");
	EventBus::new(connection).await
}

fn amqp_address() -> Result<String, EventBusError> {
	let address = std::env::var("AMQP_ADDR")?;
	Ok(address)
}
