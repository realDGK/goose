use crate::eval_suites::BenchAgentError;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

pub struct ErrorCaptureLayer {
    errors: Arc<Mutex<Vec<BenchAgentError>>>,
}

impl ErrorCaptureLayer {
    pub fn new(errors: Arc<Mutex<Vec<BenchAgentError>>>) -> Self {
        Self { errors }
    }
}

impl<S> Layer<S> for ErrorCaptureLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // Only capture error and warning level events
        if *event.metadata().level() <= tracing::Level::WARN {
            let mut visitor = JsonVisitor::new();
            event.record(&mut visitor);

            if let Some(message) = visitor.recorded_fields.get("message") {
                let error = BenchAgentError {
                    message: message.to_string(),
                    level: event.metadata().level().to_string(),
                    timestamp: Utc::now(),
                };

                let errors = self.errors.clone();
                tokio::spawn(async move {
                    let mut errors = errors.lock().await;
                    errors.push(error);
                });
            }
        }
    }
}

struct JsonVisitor {
    recorded_fields: serde_json::Map<String, serde_json::Value>,
}

impl JsonVisitor {
    fn new() -> Self {
        Self {
            recorded_fields: serde_json::Map::new(),
        }
    }
}

impl tracing::field::Visit for JsonVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.recorded_fields.insert(
            field.name().to_string(),
            serde_json::Value::String(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.recorded_fields.insert(
            field.name().to_string(),
            serde_json::Value::String(format!("{:?}", value)),
        );
    }
}
