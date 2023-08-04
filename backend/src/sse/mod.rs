use std::sync::Arc;

use crate::broadcaster::crew::CrewBroadcaster;

pub mod crew;

pub struct NotificationState {
    pub crew_broadcaster: Arc<CrewBroadcaster>,
}