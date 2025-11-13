use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AgentEvent {
    TaskCreated {
        id: Uuid,
        task_type: String,
        status: String,
    },
    TaskProgress {
        id: Uuid,
        progress: u8,
        message: String,
    },
    ContainerStatus {
        container_id: Uuid,
        status: String,
    },
}

pub fn emit(event: AgentEvent) {
    if let Ok(payload) = serde_json::to_string(&Envelope::new(event)) {
        tracing::info!(target: "orbit::events", "{}", payload);
    }
}

#[derive(Serialize)]
struct Envelope<T> {
    id: Uuid,
    timestamp: String,
    #[serde(flatten)]
    inner: T,
}

impl<T> Envelope<T> {
    fn new(inner: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default(),
            inner,
        }
    }
}
