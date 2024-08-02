use maud::{html, Markup};
use bamboo_common::core::error::{BambooError, BambooResult};
use mrml::prelude::render::RenderOptions;

pub async fn mail(
    title: impl Into<String> + Clone,
    message: Markup,
    action_label: Option<impl Into<String> + Clone>,
    action_link: Option<impl Into<String> + Clone>,
) -> BambooResult<String> {
    let template = html! {
        mjml {
            mj-head {
                mj-title {
                    (title.clone().into())
                }
                mj-attributes {
                    mj-all font-family="system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji'" {}
                    mj-text font-weight="400" font-size="16px" color="#333333" line-height="24px" font-family="system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji'" {}
                }
            }
            mj-body {
                mj-section background-url="cid:background" background-size="cover" background-repeat="no-repeat" {
                    mj-column {
                        mj-image src="cid:logo" alt="" align="center" width="64px" {}
                    }
                }
                mj-section background-color="#e3ede9" {
                    mj-column width="400px" {
                        mj-text font-size="20px" color="#333333" {
                            (title.into())
                        }
                        (message)
                        @if let (Some(link), Some(label)) = (action_link, action_label) {
                            mj-button background-color="#598c79" href=(link.into()) font-size="16px" {
                                (label.into())
                            }
                        }
                    }
                }
                mj-section {
                    mj-column width="100%" padding="0" {
                        mj-text color="#333333" font-size="11px" align="center" line-height="16px" {
                            "© Bambushain, made by Imanuel Ulbricht, Christina Ruebsam und Hans-Jürgen Wandschneider"
                        }
                    }
                }
                mj-section padding-top="0" {
                    mj-group {
                        mj-column width="100%" padding-right="0" {
                            mj-text color="#598c79" font-size="11px" align="center" line-height="16px" font-weight="bold" {
                                a style="color:#598c79;margin-right:24px" href="https://bambushain.app/legal/data-protection" {
                                    "Impressum"
                                }
                                a style="color:#598c79" href="https://bambushain.app/legal/data-protection" {
                                    "Datenschutz"
                                }
                            }
                        }
                    }
                }
            }
        }
    }.into_string();

    mrml::async_parse(template)
        .await
        .map_err(|err| BambooError::mailing(format!("Failed to parse mail {err}")))?
        .render(&RenderOptions::default())
        .map_err(|err| BambooError::mailing(format!("Failed to render mail {err}")))
}
