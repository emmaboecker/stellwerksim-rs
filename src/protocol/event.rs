use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "ereignis")]
pub struct Event {
    #[serde(rename = "zid")]
    pub id: String,
    #[serde(rename = "art")]
    pub event_type: EventType,
    pub name: String,
    #[serde(rename = "verspaetung")]
    pub delay: i32,
    #[serde(rename = "gleis")]
    pub platform: String,
    #[serde(rename = "plangleis")]
    pub scheduled_platform: String,
    #[serde(rename = "von")]
    pub origin: String,
    #[serde(rename = "nach")]
    pub destination: String,
    #[serde(rename = "sichtbar")]
    pub visible: bool,
    #[serde(rename = "amgleis")]
    pub at_platform: bool,
    #[serde(rename = "usertext")]
    pub user_text: Option<String>,
    #[serde(rename = "usertextsender")]
    pub user_text_sender: Option<String>,
    #[serde(rename = "hinweistext")]
    pub notice_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum EventType {
    #[serde(rename = "einfahrt")]
    ComingIn,
    #[serde(rename = "ankunft")]
    Arrival,
    #[serde(rename = "abfahrt")]
    Departure,
    #[serde(rename = "ausfahrt")]
    GoingOut,
    #[serde(rename = "rothalt")]
    RedStop,
    #[serde(rename = "wurdegruen")]
    TurnedGreen,
    #[serde(rename = "kuppeln")]
    Couple,
    #[serde(rename = "fluegeln")]
    Wing,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_variant::to_variant_name(self).unwrap())
    }
}

impl EventType {
    pub fn all() -> Vec<EventType> {
        vec![
            EventType::ComingIn,
            EventType::Arrival,
            EventType::Departure,
            EventType::GoingOut,
            EventType::RedStop,
            EventType::TurnedGreen,
            EventType::Couple,
            EventType::Wing,
        ]
    }
}
