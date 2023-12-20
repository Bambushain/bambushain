use bamboo_entities::prelude::Event;
use chrono::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct DayProps {
    pub day: u32,
    pub month: u32,
    pub year: i32,
    pub selected_month: u32,
    pub events: Vec<Event>,
    pub on_added: Callback<Event>,
    pub on_updated: Callback<Event>,
    pub on_deleted: Callback<Event>,
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct EventEntryProps {
    pub event: Event,
    pub on_updated: Callback<Event>,
    pub on_deleted: Callback<Event>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CalendarProps {
    pub date: NaiveDate,
}

#[derive(Properties, PartialEq, Clone)]
pub struct AddEventDialogProps {
    pub start_date: NaiveDate,
    pub on_added: Callback<Event>,
    pub on_cancel: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct EditEventDialogProps {
    pub event: Event,
    pub on_updated: Callback<Event>,
    pub on_deleted: Callback<Event>,
    pub on_cancel: Callback<()>,
}
