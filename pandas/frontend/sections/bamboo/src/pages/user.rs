use yew::prelude::*;
use yew_cosmo::prelude::*;
use yew_hooks::use_async;
use yew_hooks::use_mount;

use bamboo_common::frontend::ui::{BambooCard, BambooCardList};
use bamboo_pandas_frontend_base::controls::BambooErrorMessage;

use crate::api;

#[function_component(UsersPage)]
pub fn users_page() -> Html {
    log::debug!("Render users page");
    log::debug!("Initialize state and callbacks");
    let users_state = use_async(async move { api::get_users().await });

    {
        let users_state = users_state.clone();

        use_mount(move || {
            users_state.run();
        });
    }

    if users_state.loading {
        html!(<CosmoProgressRing />)
    } else if let Some(error) = users_state.error.clone() {
        html!(
            <BambooErrorMessage
                message="Die Pandas konnten leider nicht geladen werden"
                header="Fehler beim Laden"
                page="users"
                form="users_page"
                error={error}
            />
        )
    } else if let Some(data) = &users_state.data {
        html!(
            <>
                <CosmoTitle title="Pandas" />
                <BambooCardList>
                    { for data.iter().map(|user|
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
                    ) }
                </BambooCardList>
            </>
        )
    } else {
        html!()
    }
}
