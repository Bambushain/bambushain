use std::sync::Arc;

use crate::broadcaster::calendar::CalendarBroadcaster;
use crate::broadcaster::crew::CrewBroadcaster;
use crate::broadcaster::kill::KillBroadcaster;
use crate::broadcaster::mount::MountBroadcaster;
use crate::broadcaster::savage_mount::SavageMountBroadcaster;

pub mod calendar;
pub mod crew;
pub mod kill;
pub mod mount;
pub mod savage_mount;

pub struct NotificationState {
    pub calendar_broadcaster: Arc<CalendarBroadcaster>,
    pub crew_broadcaster: Arc<CrewBroadcaster>,
    pub kill_broadcaster: Arc<KillBroadcaster>,
    pub mount_broadcaster: Arc<MountBroadcaster>,
    pub savage_mount_broadcaster: Arc<SavageMountBroadcaster>,
}