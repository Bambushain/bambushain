use std::borrow::Cow;

use wasm_bindgen::prelude::*;
use web_sys::EventSource;
use yew::prelude::*;
use yew_hooks::{use_mount, use_unmount};

#[hook]
pub fn use_event_source<E, F>(url: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(()) + 'static,
{
    #[derive(PartialEq, Clone)]
    struct EventSourceDeps {
        url: Cow<'static, str>,
        callback: Callback<()>,
    }

    let deps = EventSourceDeps {
        url: url.into(),
        callback: Callback::from(callback),
    };

    use_mount(move || {
        log::debug!("Start event source {}", &deps.url);
        let source = EventSource::new(&deps.url);
        let event_source = match source {
            Ok(source) => {
                let unmount_source = source.clone();
                use_unmount(move || unmount_source.close());
                source
            }
            Err(err) => {
                log::warn!("Failed to start event source, no SSE for you: {err:?}");
                return;
            }
        };

        log::debug!("Event source connected");
        let message_handler: Closure<dyn Fn()> = Closure::new(move || {
            deps.callback.emit(());
        });
        event_source.set_onmessage(Some(message_handler.as_ref().unchecked_ref()));
        message_handler.forget();
        log::debug!("Waiting for event source to get data");
    })
}
