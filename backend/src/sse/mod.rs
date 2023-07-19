use std::sync::Arc;

use crate::broadcaster::calendar::CalendarBroadcaster;

pub mod calendar;

pub struct NotificationState {
    pub calendar_broadcaster: Arc<CalendarBroadcaster>,
}