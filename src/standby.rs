use crate::Error;
use dashmap::DashMap;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::protocol::Event;

#[derive(Debug)]
pub struct Standby {
    events: DashMap<String, UnboundedSender<Event>>,
}

impl Standby {
    pub fn new() -> Self {
        Standby {
            events: DashMap::new(),
        }
    }

    pub fn receive_events(&self, train_id: String) -> UnboundedReceiver<Event> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let _ = self.events.insert(train_id, tx);
        rx
    }

    pub fn process_event(&self, event: Event) -> Result<(), Error> {
        if let Some(tx) = self.events.get(&event.id) {
            tx.send(event.clone())
                .map_err(|_| Error::ChannelError(SendError(event.name.to_owned())))?;
        }
        Ok(())
    }
}

impl Default for Standby {
    fn default() -> Self {
        Self::new()
    }
}
