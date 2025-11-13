use serde::Serialize;
use time::OffsetDateTime;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub timestamp: String,
    #[serde(flatten)]
    pub payload: AgentEvent,
}

impl EventEnvelope {
    fn new(payload: AgentEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default(),
            payload,
        }
    }
}

#[derive(Clone)]
pub struct EventHub {
    sender: broadcast::Sender<EventEnvelope>,
}

impl EventHub {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn emit(&self, event: AgentEvent) {
        let envelope = EventEnvelope::new(event);
        if let Err(error) = self.sender.send(envelope.clone()) {
            tracing::warn!(?error, "No se pudo entregar evento a los suscriptores");
        }

        if let Ok(payload) = serde_json::to_string(&envelope) {
            tracing::info!(target: "orbit::events", "{}", payload);
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }
}
