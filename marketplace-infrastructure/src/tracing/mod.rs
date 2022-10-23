use anyhow::Result;
use opentelemetry::{
	global::shutdown_tracer_provider,
	sdk::trace::{self, RandomIdGenerator, Sampler},
};
use opentelemetry_datadog::ApiVersion;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

mod datadog_event_format;

pub struct Tracer;

impl Tracer {
	pub fn init(service_name: &str) -> Result<Self> {
		// Install a new OpenTelemetry trace pipeline
		let otel_tracer = opentelemetry_datadog::new_pipeline()
			.with_service_name(service_name)
			.with_version(ApiVersion::Version05)
			.with_trace_config(
				trace::config()
					.with_sampler(Sampler::AlwaysOn)
					.with_id_generator(RandomIdGenerator::default()),
			)
			.install_batch(opentelemetry::runtime::Tokio)?;

		// Create a tracing layer with the configured tracer
		let telemetry = tracing_opentelemetry::layer().with_tracer(otel_tracer);

		let subscriber = tracing_subscriber::fmt::Subscriber::builder()
			// subscriber configuration
			.with_env_filter(EnvFilter::from_default_env())
			.with_max_level(tracing::Level::TRACE)
			.with_ansi(false)
			.event_format(datadog_event_format::TraceIdFormat)
			.finish()
			.with(telemetry);

		// Trace executed code
		tracing::subscriber::set_global_default(subscriber)?;

		LogTracer::init()?;

		Ok(Tracer {})
	}
}

impl Drop for Tracer {
	fn drop(&mut self) {
		shutdown_tracer_provider();
	}
}
