use crate::api;
use crate::api::BannedStatus;
use crate::state::grove::{use_groves, GrovesAtom};
use bamboo_common::core::entities::user::{GroveUser, JoinStatus};
use bamboo_common::frontend::api::ApiError;
use bamboo_common::frontend::ui::{BambooCard, BambooCardList};
use bamboo_pandas_frontend_base::controls::{use_events, Calendar};
use bamboo_pandas_frontend_base::error;
use bamboo_pandas_frontend_base::routing::{AppRoute, GroveRoute};
use bamboo_pandas_frontend_base::storage::CurrentUser;
use bounce::helmet::Helmet;
use bounce::use_atom;
use chrono::Datelike;
use std::ops::Deref;
use stylist::yew::use_style;
use yew::prelude::*;
use yew::virtual_dom::Key;
use yew_autoprops::autoprops;
use yew_cosmo::prelude::*;
use yew_hooks::{use_async, use_async_with_options, use_bool_toggle, use_list, UseAsyncOptions};
use yew_router::hooks::use_navigator;
use yew_router::prelude::Redirect;

#[autoprops]
#[function_component(GroveCalendar)]
fn grove_calendar(id: i32) -> Html {
    log::debug!("Render calendar page");
    let calendar_container_style = use_style!(
        r#"
height: calc(var(--page-height) - var(--title-font-size) - var(--tab-links-height) - var(--tab-gap) - 0.5rem);
    "#
    );

    let today = chrono::Local::now().date_naive().with_day(1).unwrap();
    log::info!("Load for today {today}");

    let events = use_events(today, Some(id));

    {
        let events = events.clone();

        use_effect_with(id, move |_| {
            events.grove_id_state.set(Some(id));

            || {}
        });
    }

    html!(
        <div class={calendar_container_style}>
            <Calendar
                grove_id={Some(id)}
                events={events.events_list.current().deref().clone()}
                date={*events.date_state}
                on_navigate={events.on_navigate}
            />
        </div>
    )
}

#[autoprops]
#[function_component(Users)]
fn users(id: i32) -> Html {
    log::debug!("Render users page");
    log::debug!("Initialize state and callbacks");
    let bamboo_error_state = use_state_eq(ApiError::default);

    let unreported_error_toggle = use_bool_toggle(false);
    let selected_user_state = use_state_eq(|| None as Option<GroveUser>);

    let current_user_atom = use_atom::<CurrentUser>();

    let users_state = {
        let bamboo_error_state = bamboo_error_state.clone();

        let unreported_error_toggle = unreported_error_toggle.clone();

        use_async(async move {
            unreported_error_toggle.set(false);

            api::get_users(id, BannedStatus::Unbanned)
                .await
                .map_err(|err| {
                    bamboo_error_state.set(err.clone());
                    unreported_error_toggle.set(true);

                    err
                })
        })
    };

    let ban_user_state = {
        let selected_user_state = selected_user_state.clone();

        let users_state = users_state.clone();

        use_async(async move {
            if let Some(user) = (*selected_user_state).clone() {
                let res = api::ban_user(id, user.id).await;
                selected_user_state.set(None);
                users_state.run();
                res
            } else {
                Ok(())
            }
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
    let open_ban_user = use_callback(
        selected_user_state.clone(),
        |user: GroveUser, selected_user_state| {
            selected_user_state.set(Some(user));
        },
    );
    let ban_user = use_callback(ban_user_state.clone(), |_, ban_user_state| {
        ban_user_state.run()
    });
    let close_ban = use_callback(selected_user_state.clone(), |_, selected_user_state| {
        selected_user_state.set(None)
    });

    {
        let users_state = users_state.clone();

        use_effect_with(id, move |_| {
            users_state.run();

            || {}
        });
    }

    if users_state.loading {
        html!(<CosmoProgressRing />)
    } else if users_state.error.is_some() {
        html!(
            if *unreported_error_toggle {
                <CosmoMessage
                    header="Fehler beim Laden"
                    message="Die Pandas konnten nicht geladen werden"
                    message_type={CosmoMessageType::Negative}
                    actions={html!(<CosmoButton label="Fehler melden" on_click={report_unknown_error} />)}
                />
            } else {
                <CosmoMessage
                    header="Fehler beim Laden"
                    message="Die Pandas konnten nicht geladen werden"
                    message_type={CosmoMessageType::Negative}
                />
            }
        )
    } else if let Some(data) = &users_state.data.clone() {
        let current_user_id = current_user_atom.profile.id;
        let current_user_is_mod_in_grove = data
            .iter()
            .any(|user| user.id == current_user_atom.profile.id && user.is_mod);

        html!(
            <>
                <BambooCardList>
                    { for data.iter().cloned().map(|user|
                        {
                            let profile_picture = format!(
                                "/api/user/{}/picture#time={}",
                                user.id,
                                chrono::offset::Local::now().timestamp_millis()
                            );
                            let open_ban_user = open_ban_user.clone();
                            let user_to_ban = user.clone();

                            html!(
                                <BambooCard title={user.display_name.clone()} prepend={html!(<img style="max-height:7rem;" src={profile_picture} />)} buttons={html!(
                                    if current_user_is_mod_in_grove {
                                        <CosmoButton on_click={move |_| open_ban_user.emit(user_to_ban.clone())} label={format!("{} bannen", user.display_name.clone())} enabled={user.id != current_user_id} />
                                    }
                                )}>
                                    <CosmoAnchor href={format!("mailto:{}", user.email.clone())}>{user.email.clone()}</CosmoAnchor>
                                    if !user.discord_name.is_empty() {
                                        <span>{"Auf Discord bekannt als "}<CosmoStrong>{user.discord_name.clone()}</CosmoStrong></span>
                                    }
                                </BambooCard>
                            )
                        }
                    ) }
                </BambooCardList>
                if let Some(user) = (*selected_user_state).clone() {
                    <CosmoConfirm
                        confirm_type={CosmoModalType::Negative}
                        title={format!("{} bannen", user.display_name.clone())}
                        message={format!("Soll der Panda {} wirklich gebannt werden?", user.display_name.clone())}
                        confirm_label={format!("{} bannen", user.display_name.clone())}
                        decline_label={format!("{} nicht bannen", user.display_name.clone())}
                        on_confirm={ban_user}
                        on_decline={close_ban.clone()}
                    />
                }
            </>
        )
    } else {
        html!()
    }
}

#[autoprops]
#[function_component(Management)]
fn management(
    id: i32,
    name: &AttrValue,
    on_invite_changed: &Callback<()>,
    invite_link: &Option<AttrValue>,
) -> Html {
    let groves_atom = use_groves();

    let mod_list = use_list(vec![]);

    let delete_grove_open_toggle = use_bool_toggle(false);

    let user_to_unban_state = use_state_eq(|| None as Option<GroveUser>);

    let current_user_atom = use_atom::<CurrentUser>();

    let navigator = use_navigator().unwrap();

    let users_state = {
        let mod_list = mod_list.clone();

        use_async(async move {
            api::get_users(id, BannedStatus::All).await.map(|pandas| {
                mod_list.set(
                    pandas
                        .iter()
                        .filter_map(|item| if item.is_mod { Some(item.id) } else { None })
                        .collect(),
                );
                pandas
            })
        })
    };
    let save_mods_state = {
        let mod_list = mod_list.clone();

        let current_user_atom = current_user_atom.clone();

        use_async(async move {
            let mods = mod_list.current();

            api::save_grove_mods(
                id,
                &mods
                    .deref()
                    .iter()
                    .cloned()
                    .filter(|user_id| *user_id != current_user_atom.profile.id)
                    .collect::<Vec<_>>(),
            )
            .await
        })
    };
    let delete_grove_state = {
        let navigator = navigator.clone();
        use_async(async move {
            let res = api::delete_grove(id).await;
            if res.is_ok() {
                yew::platform::spawn_local(async move {
                    if let Ok(groves) = api::get_groves().await {
                        groves_atom.set(GrovesAtom { groves })
                    }

                    navigator.push(&AppRoute::GrovesRoot);
                });
            }

            res
        })
    };
    let unban_user_state = {
        let users_state = users_state.clone();
        let user_to_unban_state = user_to_unban_state.clone();

        use_async(async move {
            if let Some(user) = (*user_to_unban_state).clone() {
                let res = api::unban_user(id, user.id).await;
                if res.is_ok() {
                    users_state.run();
                    user_to_unban_state.set(None);
                }

                res
            } else {
                Ok(())
            }
        })
    };

    {
        let users_state = users_state.clone();

        use_effect_with(id, move |_| {
            users_state.run();

            || {}
        });
    }

    let management_content_style = use_style!(
        r#"
height: calc(var(--page-height) - var(--title-font-size) - var(--tab-links-height) - var(--tab-gap) - 2rem);
width: 50%;
        "#
    );

    let save_mods = use_callback(save_mods_state.clone(), |_, save_mods_state| {
        save_mods_state.run();
    });
    let mod_deselect = use_callback(mod_list.clone(), |id: AttrValue, mod_list| {
        mod_list.retain(|item| item != &id.to_string().parse::<i32>().unwrap());
    });
    let mod_select = use_callback(mod_list.clone(), |id: AttrValue, mod_list| {
        mod_list.push(id.to_string().parse::<i32>().unwrap());
    });
    let delete_grove = use_callback(delete_grove_state.clone(), |_, delete_grove_state| {
        delete_grove_state.run()
    });
    let close_delete = use_callback(
        delete_grove_open_toggle.clone(),
        |_, delete_grove_open_toggle| {
            delete_grove_open_toggle.set(false);
        },
    );
    let open_delete = use_callback(
        delete_grove_open_toggle.clone(),
        |_, delete_grove_open_toggle| {
            delete_grove_open_toggle.set(true);
        },
    );

    let unban_user = use_callback(unban_user_state.clone(), |_, unban_user_state| {
        unban_user_state.run()
    });
    let close_unban = use_callback(user_to_unban_state.clone(), |_, user_to_unban_state| {
        user_to_unban_state.set(None);
    });
    let open_unban = use_callback(
        user_to_unban_state.clone(),
        |user: GroveUser, user_to_unban_state| {
            user_to_unban_state.set(Some(user));
        },
    );

    let enable_invite = use_callback(on_invite_changed.clone(), move |_, on_invite_changed| {
        let on_invite_changed = on_invite_changed.clone();

        yew::platform::spawn_local(async move {
            if api::enable_invite(id.clone()).await.is_ok() {
                on_invite_changed.emit(())
            }
        });
    });
    let disable_invite = use_callback(on_invite_changed.clone(), move |_, on_invite_changed| {
        let on_invite_changed = on_invite_changed.clone();

        yew::platform::spawn_local(async move {
            if api::disable_invite(id.clone()).await.is_ok() {
                on_invite_changed.emit(())
            }
        });
    });

    let current_user_id = current_user_atom.profile.id;
    let mod_items = if let Some(users) = &users_state.data.clone() {
        users
            .iter()
            .filter_map(|user| {
                if user.id != current_user_id && !user.is_banned {
                    Some(CosmoModernSelectItem::new(
                        user.display_name.clone(),
                        user.id.to_string(),
                        mod_list.current().contains(&user.id),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    html!(
        <div class={management_content_style}>
            <CosmoHeader
                level={CosmoHeaderLevel::H1}
                header={format!("Willkommen in der Verwaltung von {name}")}
            />
            <CosmoParagraph>
                { "Hier hast du die Möglichkeit deinen Hain zu verwalten. Unten findest du den Einladungslink damit Leute deinem Hain beitreten können." }
                <br />
                { "Außerdem hast du die Möglichkeit Mods festzulegen. Mods können andere Mods ernennen oder ihnen die Rechte nehmen." }
                <br />
                { "Dazu haben Mods die Rechte Pandas aus einem Hain zu werfen." }
                <br />
                { "Daneben können Mods den Hain auch umbennen und vor allem löschen." }
            </CosmoParagraph>
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Pandas einladen" />
            if let Some(invite_link) = invite_link {
                <CosmoParagraph>
                    { "Dein Hain ist ziemlich sinnlos ohne andere Pandas, deswegen kannst du andere Pandas mit dem Link hier direkt in deinen Hain einladen." }
                    <br />
                    { "Einfach kopieren und verschicken, anschließend können andere Pandas deinem Hain beitreten." }
                    <br />
                    <CosmoAnchor href={invite_link.clone()}>
                        { format!("https://bambushain.app{invite_link}") }
                    </CosmoAnchor>
                </CosmoParagraph>
                <CosmoParagraph>
                    { "Wenn du nicht möchtest, dass weitere Pandas in deinen Hain kommen, kannst du mit einem Klick auf den Button Einladungen deaktivieren." }
                    <br />
                    <CosmoButton label="Einladungen deaktivieren" on_click={disable_invite} />
                </CosmoParagraph>
            } else {
                <CosmoParagraph>
                    { "So wie es aussieht hast du Einladungen deaktiviert, wenn du diese aktivieren willst, klick einfach unten auf den Button." }
                    <br />
                    <CosmoButton label="Einladungen aktivieren" on_click={enable_invite} />
                </CosmoParagraph>
            }
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Modverwaltung" />
            <CosmoParagraph>
                { "Hier hast du die Möglichkeit die Mods zu verwalten, wähle einfach alle Pandas aus, die du als Mods in deinem Hain neben dir haben willst." }
            </CosmoParagraph>
            <CosmoForm
                buttons={html!(<CosmoButton is_submit={true} label="Mods speichern" />)}
                on_submit={save_mods}
            >
                <CosmoModernSelect
                    label="Mods"
                    items={mod_items}
                    on_select={mod_select}
                    on_deselect={mod_deselect}
                />
            </CosmoForm>
            if let Some(users) = &users_state.data.clone() {
                if users.iter().any(|user| user.is_banned) {
                    <CosmoHeader level={CosmoHeaderLevel::H3} header="Gebannte Pandas" />
                    <CosmoParagraph>
                        { "So wie es aussieht hattest du schon einmal Probleme mit anderen Pandas in deinem Hain." }
                        <br />
                        { "Das ist echt schade. Aber vielleicht hat sich die Situation schon wieder gebessert, falls ja, kannst du unten den Panda wieder entbannen." }
                    </CosmoParagraph>
                    <CosmoTable
                        headers={vec![AttrValue::from("Name"), AttrValue::from("Email"), AttrValue::from("Discord"), AttrValue::from("Aktionen")]}
                    >
                        { for users.iter().filter_map(|user| {
                            let open_unban = open_unban.clone();

                            let user_to_unban = user.clone();

                            if user.is_banned {
                                Some(CosmoTableRow::from_table_cells(vec![
                                    CosmoTableCell::from_html(html!(user.display_name.clone()), Some(Key::from(0))),
                                    CosmoTableCell::from_html(html!(user.email.clone()), Some(Key::from(1))),
                                    CosmoTableCell::from_html(html!(user.discord_name.clone()), Some(Key::from(2))),
                                    CosmoTableCell::from_html(html!(
                                        <CosmoButton label="Ban aufheben" on_click={move |_| open_unban.emit(user_to_unban.clone())} />
                                    ), Some(Key::from(3))),
                                ], Some(Key::from(user.id))))
                            } else {
                                None
                            }
                        }) }
                    </CosmoTable>
                }
            }
            <CosmoHeader level={CosmoHeaderLevel::H2} header="Gefahrenzone" />
            <CosmoMessage
                message_type={CosmoMessageType::Negative}
                header="Achtung!"
                message="Achtung, hier beginnt die Gefahrenzone, wenn du unten auf Hain löschen klickst wird dein Hain gelöscht. Du wirst nochmal um eine Bestätigung gebeten, danach ist der Hain unwiderbringlich gelöscht."
                actions={html!(<CosmoButton label="Hain löschen" on_click={open_delete} />)}
            />
            if *delete_grove_open_toggle {
                <CosmoConfirm
                    confirm_type={CosmoModalType::Negative}
                    title="Hain löschen"
                    message={format!("Soll der Hain {} wirklich gelöscht werden? Dies löscht auch den Eventkalender.", name.clone())}
                    confirm_label="Hain löschen"
                    decline_label="Hain nicht löschen"
                    on_confirm={delete_grove}
                    on_decline={close_delete.clone()}
                />
            }
            if *delete_grove_open_toggle && delete_grove_state.error.is_some() {
                <CosmoAlert
                    alert_type={CosmoModalType::Negative}
                    title="Fehler beim Löschen"
                    message={format!("Der Hain {} konnte nicht gelöscht werden. Bitte wende dich an den Bambussupport.", name.clone())}
                    close_label="Verstanden"
                    on_close={close_delete}
                />
            }
            if let Some(user_to_unban) = (*user_to_unban_state).clone() {
                <CosmoConfirm
                    confirm_type={CosmoModalType::Positive}
                    title="Ban aufheben"
                    message={format!("Soll der Ban von {} wirklich aufgehoben werden? Anschließend kann {} wieder beitreten.", user_to_unban.display_name.clone(), user_to_unban.display_name.clone())}
                    confirm_label="Ban aufheben"
                    decline_label="Ban nicht aufheben"
                    on_confirm={unban_user}
                    on_decline={close_unban.clone()}
                />
                if unban_user_state.error.is_some() {
                    <CosmoAlert
                        alert_type={CosmoModalType::Negative}
                        title="Fehler beim Ban aufheben"
                        message={format!("Der Ban von {} konnte leider nicht aufgehoben werden. Bitte wende dich an den Bambussupport.", user_to_unban.display_name.clone())}
                        close_label="Verstanden"
                        on_close={close_unban}
                    />
                }
            }
        </div>
    )
}

#[autoprops]
#[function_component(GroveDetailsPage)]
pub fn grove_details(id: i32, name: AttrValue) -> Html {
    let grove_state = use_async(async move { api::get_grove(id).await });

    let selected_index_state = use_state_eq(|| 0usize);

    let user_is_mod_toggle = use_bool_toggle(false);

    let current_user_atom = use_atom::<CurrentUser>();

    {
        let grove_state = grove_state.clone();
        let user_is_mod_toggle = user_is_mod_toggle.clone();

        use_effect_with(id, move |_| {
            grove_state.run();
            user_is_mod_toggle.set(false);

            yew::platform::spawn_local(async move {
                if let Ok(users) = api::get_users(id, BannedStatus::Unbanned).await {
                    user_is_mod_toggle.set(
                        users
                            .iter()
                            .any(|user| user.id == current_user_atom.profile.id && user.is_mod),
                    );
                }
            });

            || {}
        });
    }
    let invite_changed_callback = use_callback(grove_state.clone(), |_, grove_state| {
        grove_state.run();
    });
    let select_item = use_callback(selected_index_state.clone(), |idx: usize, state| {
        state.set(idx)
    });

    let mut items = vec![
        CosmoTabItem::from_label_and_children(
            "Hainkalender".into(),
            html!(<GroveCalendar id={id}/>),
        ),
        CosmoTabItem::from_label_and_children("Pandas".into(), html!(<Users id={id}/>)),
    ];

    if *user_is_mod_toggle {
        if let Some(grove) = &grove_state.data.clone() {
            items.push(CosmoTabItem::from_label_and_children(
                "Verwaltung".into(),
                html!(
                    <Management
                        id={id}
                        name={name.clone()}
                        invite_link={grove.get_invite_link()}
                        on_invite_changed={invite_changed_callback}
                    />
                ),
            ));
        }
    }

    html!(
        <>
            <Helmet>
                <title>{ name.clone() }</title>
            </Helmet>
            <CosmoTitle title={name.clone()} />
            if grove_state.error.is_some() {
                <Redirect<AppRoute> to={AppRoute::GrovesRoot} />
            } else if grove_state.loading {
                <CosmoProgressRing />
            } else {
                <CosmoTabControl
                    on_select_item={select_item}
                    selected_index={*selected_index_state}
                >
                    { items.clone() }
                </CosmoTabControl>
            }
        </>
    )
}

#[autoprops]
#[function_component(AddGrovePage)]
pub fn add_grove() -> Html {
    let name_state = use_state_eq(|| AttrValue::from(""));

    let groves_atom = use_groves();

    let invite_on_toggle = use_bool_toggle(true);

    let navigator = use_navigator().unwrap();

    let create_grove_state = {
        let name_state = name_state.clone();

        let invite_on_toggle = invite_on_toggle.clone();

        use_async(async move {
            let res = api::create_grove((*name_state).to_string(), *invite_on_toggle).await;
            if let Ok(res) = res.clone() {
                name_state.set("".into());
                invite_on_toggle.set(true);
                let mut groves = groves_atom.groves.clone();
                groves.push(res.clone());

                groves_atom.set(GrovesAtom { groves });

                navigator.push(&GroveRoute::Grove {
                    id: res.id,
                    name: res.name.clone(),
                });
            }

            res
        })
    };

    let create_grove = use_callback(create_grove_state.clone(), |_, create_grove_state| {
        create_grove_state.run();
    });
    let invite_on_check = use_callback(invite_on_toggle.clone(), |value, invite_on_toggle| {
        invite_on_toggle.set(value)
    });
    let name_input = use_callback(name_state.clone(), |value, name_state| {
        name_state.set(value)
    });

    let content_style = use_style!(
        r#"
height: calc(var(--page-height) - var(--title-font-size) - var(--tab-links-height) - var(--tab-gap) - 2rem);
width: min(50rem, 50%);
        "#
    );

    html!(
        <div class={content_style}>
            <Helmet>
                <title>{ "Neuer Hain" }</title>
            </Helmet>
            <CosmoTitle title="Neuer Hain" />
            <CosmoParagraph>
                { "Cool, dass du deinen eigenen Hain erstellen möchtest. Dafür brauchen wir zwei kleine Infos von dir, einmal einen Namen und die Bestätigung, dass andere Pandas in den Hain eingeladen werden können. Füll das Formular unten einfach aus, klick auf Hain erstellen und schon bist du fertig." }
            </CosmoParagraph>
            if create_grove_state.error.is_some() {
                <CosmoMessage
                    header="Fehler beim Erstellen"
                    message="Tut uns leid, der Hain konnte leider nicht erstellt werden. Bitte wende dich an den Bambussupport"
                />
            }
            <CosmoForm
                on_submit={create_grove}
                buttons={html!(<CosmoButton is_submit={true} label="Hain erstellen" />)}
            >
                <CosmoTextBox label="Name" value={(*name_state).clone()} on_input={name_input} />
                <CosmoSwitch
                    label="Einladungen aktiv"
                    checked={*invite_on_toggle}
                    on_check={invite_on_check}
                />
            </CosmoForm>
        </div>
    )
}

#[autoprops]
#[function_component(GroveInvitePage)]
pub fn grove_invite(id: i32, name: AttrValue, invite_secret: AttrValue) -> Html {
    let navigator = use_navigator().unwrap();

    let join_grove_state = {
        let invite_secret = invite_secret.clone();
        let name = name.clone();

        let navigator = navigator.clone();

        use_async(async move {
            let res = api::join_grove(id, invite_secret.to_string()).await;
            if res.is_ok() {
                navigator.push(&GroveRoute::Grove {
                    id,
                    name: name.to_string(),
                })
            }

            res
        })
    };

    let check_join_status_state = use_async_with_options(
        async move { api::check_join_status(id).await },
        UseAsyncOptions::enable_auto(),
    );

    let join_grove = use_callback(join_grove_state.clone(), |_, join_grove_state| {
        join_grove_state.run()
    });
    let dont_join_grove = use_callback(navigator.clone(), |_, navigator| {
        navigator.push(&AppRoute::Home)
    });

    html!(
        if check_join_status_state.loading {
            <CosmoModal title="Lädt..." buttons={html!()}>
                <CosmoProgressRing />
            </CosmoModal>
        } else if let Some(status) = &check_join_status_state.data {
            if status == &JoinStatus::Banned {
                <CosmoAlert
                    title="Gebannt"
                    message={format!("Du wurdest aus dem Hain {name} gebannt und kannst nicht beitreten.")}
                    close_label="Zum Kalender"
                    on_close={dont_join_grove.clone()}
                />
            } else if status == &JoinStatus::Joined {
                <Redirect<GroveRoute> to={GroveRoute::Grove {id, name: name.to_string()}} />
            } else if status == &JoinStatus::NotJoined {
                <CosmoConfirm
                    title={format!("{name} beitreten")}
                    message={format!("Du wurdest eingeladen dem Hain {name} beizutreten. Wenn du das machst hast du Zugriff auf den gemeinsamen Kalender und die Pandaliste.")}
                    on_confirm={join_grove}
                    on_decline={dont_join_grove}
                    confirm_label="Hain beitreten"
                    decline_label="Hain nicht beitreten"
                />
            }
        } else if check_join_status_state.error.is_some() {
            <Redirect<AppRoute> to={AppRoute::Home} />
        }
    )
}
