use std::ops::Deref;

use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::use_mount;
use yew_hooks::{use_async, use_bool_toggle};

use bamboo_common::frontend::api::ApiError;
use bamboo_common::frontend::ui::{BambooCard, BambooCardList};
use bamboo_pandas_frontend_base::error;

use crate::api;

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    log::debug!("Render users page");
    log::debug!("Initialize state and callbacks");
    let bamboo_error_state = use_state_eq(ApiError::default);

    let unreported_error_toggle = use_bool_toggle(false);

    let users_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            unreported_error_toggle.set(false);

            api::get_users().await.map_err(|err| {
                bamboo_error_state.set(err.clone());
                unreported_error_toggle.set(true);

                err
            })
        })
    };

    let report_unknown_error = use_callback(
        (bamboo_error_state.clone(), unreported_error_toggle.clone()),
        |_, (bamboo_error_state, unreported_error_toggle)| {
            error::report_unknown_error(
                "bamboo_user",
                "users_page",
                bamboo_error_state.deref().clone(),
            );
            unreported_error_toggle.set(false);
        },
    );

    {
        let users_state = users_state.clone();

        use_mount(move || {
            users_state.run();
        });
    }

    if users_state.loading {
        html!(
            <CosmoProgressRing />
        )
    } else if users_state.error.is_some() {
        if *unreported_error_toggle {
            html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Pandas konnten nicht geladen werden" message_type={CosmoMessageType::Negative} actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)} />
            )
        } else {
            html!(
                <CosmoMessage header="Fehler beim Laden" message="Die Pandas konnten nicht geladen werden" message_type={CosmoMessageType::Negative} />
            )
        }
    } else if let Some(data) = &users_state.data {
        html!(
            <>
                <CosmoTitle title="Pandas" />
                <BambooCardList>
                    {for data.iter().map(|user|
                        {
                            let profile_picture = format!(
                                "/api/user/{}/picture#time={}",
                                user.id,
                                chrono::offset::Local::now().timestamp_millis()
                            );
                            html!(
                                <BambooCard title={user.display_name.clone()} prepend={html!(<img style="max-height:7rem;" src={profile_picture} />)}>
                                    <CosmoAnchor href={format!("mailto:{}", user.email.clone())}>{user.email.clone()}</CosmoAnchor>
                                    if !user.discord_name.is_empty() {
                                        <span>{"Auf Discord bekannt als "}<CosmoStrong>{user.discord_name.clone()}</CosmoStrong></span>
                                    }
                                </BambooCard>
                            )
                        }
                    )}
                </BambooCardList>
            </>
        )
    } else {
        html!()
    }
}
