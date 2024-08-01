use maud::{html, Markup, DOCTYPE};

fn head(title: impl Into<String>) -> Markup {
    html! {
        head {
            title {
                (title.into())
            }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width";
            style type="text/css" {
                r#"
                #ko_onecolumnBlock_1 .textintenseStyle a, #ko_onecolumnBlock_1 .textintenseStyle a:link, #ko_onecolumnBlock_1 .textintenseStyle a:visited, #ko_onecolumnBlock_1 .textintenseStyle a:hover{
                    color: #ffffff;
                    color: ;
                    text-decoration: none;
                    font-weight: bold;
                    text-decoration: none
                }

                #ko_onecolumnBlock_1 .textlightStyle a, #ko_onecolumnBlock_1 .textlightStyle a:link, #ko_onecolumnBlock_1 .textlightStyle a:visited, #ko_onecolumnBlock_1 .textlightStyle a:hover{
                    color: #3F3D33;
                    color: ;

                    text-decoration: none;
                    font-weight: bold;
                    text-decoration:
                }

                /* CLIENT-SPECIFIC STYLES */
                #outlook a {
                    padding: 0;
                } /* Force Outlook to provide a "view in browser" message */
                .ReadMsgBody{
                    width: 100%;
                }
                .ExternalClass {
                    width: 100%;
                } /* Force Hotmail to display emails at full width */
                .ExternalClass,
                .ExternalClass p,
                .ExternalClass span,
                .ExternalClass font,
                .ExternalClass td,
                .ExternalClass div {
                    line-height: 100%;
                } /* Force Hotmail to display normal line spacing */
                body,
                table,
                td,
                a {
                    -webkit-text-size-adjust: 100%;
                    -ms-text-size-adjust: 100%;
                } /* Prevent WebKit and Windows mobile changing default text sizes */
                table,
                td {
                    mso-table-lspace: 0pt;
                    mso-table-rspace: 0pt;
                } /* Remove spacing between tables in Outlook 2007 and up */
                img {
                    -ms-interpolation-mode: bicubic;
                } /* Allow smoother rendering of resized image in Internet Explorer */

                /* RESET STYLES */
                body {
                    margin: 0;
                    padding: 0;
                }
                img {
                    border: 0;
                    height: auto;
                    line-height: 100%;
                    outline: none;
                    text-decoration: none;
                }
                table {
                    border-collapse: collapse !important;
                }
                body {
                    height: 100% !important;
                    margin: 0;
                    padding: 0;
                    width: 100% !important;
                }

                /* iOS BLUE LINKS */
                .appleBody a {
                    color: #68440a;
                    text-decoration: none;
                }
                .appleFooter a {
                    color: #999999;
                    text-decoration: none;
                }

                /* MOBILE STYLES */
                @media screen and (max-width: 525px) {
                    /* ALLOWS FOR FLUID TABLES */
                    table[class="wrapper"] {
                        width: 100% !important;
                        min-width: 0px !important;
                    }

                    /* USE THESE CLASSES TO HIDE CONTENT ON MOBILE */
                    td[class="mobile-hide"] {
                        display:none;
                    }

                    img[class="mobile-hide"] {
                        display: none !important;
                    }

                    img[class="img-max"] {
                        width: 100% !important;
                        max-width: 100% !important;
                        height: auto !important;
                    }

                    /* FULL-WIDTH TABLES */
                    table[class="responsive-table"] {
                        width: 100% !important;
                    }

                    /* UTILITY CLASSES FOR ADJUSTING PADDING ON MOBILE */
                    td[class="padding"] {
                        padding: 10px 5% 15px 5% !important;
                    }

                    td[class="padding-copy"] {
                        padding: 10px 5% 10px 5% !important;
                        text-align: center;
                    }

                    td[class="padding-meta"] {
                        padding: 30px 5% 0px 5% !important;
                        text-align: center;
                    }

                    td[class="no-pad"] {
                        padding: 0 0 0px 0 !important;
                    }

                    td[class="no-padding"] {
                        padding: 0 !important;
                    }

                    td[class="section-padding"] {
                        padding: 10px 15px 10px 15px !important;
                    }

                    td[class="section-padding-bottom-image"] {
                        padding: 10px 15px 0 15px !important;
                    }

                    /* ADJUST BUTTONS ON MOBILE */
                    td[class="mobile-wrapper"] {
                        padding: 10px 5% 15px 5% !important;
                    }

                    table[class="mobile-button-container"] {
                        margin:0 auto;
                        width:100% !important;
                    }

                    a[class="mobile-button"] {
                        width: 80% !important;
                        padding: 15px !important;
                        border: 0 !important;
                        font-size: 16px !important;
                    }
                }
                "#
            }
        }
    }
}

fn action(label: impl Into<String>, link: impl Into<String>) -> Markup {
    html! {
        tr {
            td {
                table width="100%" border="0" cellspacing="0" cellpadding="0" class="mobile-button-container" {
                    tbody {
                        tr {
                            td align="center" style="padding: 25px 0 0 0" class="padding-copy" {
                                table border="0" cellspacing="0" cellpadding="0" class="responsive-table" {
                                    tbody {
                                        tr {
                                            td align="center" {
                                                a target="_new" class="mobile-button" style="display: inline-block; font-size: 18px; font-weight: normal; color: #ffffff; text-decoration: none; background-color: #598c79; padding-top: 15px; padding-bottom: 15px; padding-left: 25px; padding-right: 25px; border-radius: 3px; -webkit-border-radius: 3px; -moz-border-radius: 3px; border-bottom: 3px solid #416759" href=(link.into()) {
                                                    (label.into())
                                                }
                                            }
                                        }
                                   }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn mail_title() -> Markup {
    html! {
        table border="0" cellpadding="0" cellspacing="0" width="100%" id="ko_titleBlock_1" {
            tbody {
                tr class="row-a" {
                    td bgcolor="#598c79" align="center" class="section-padding" style="padding: 15px" {
                        table border="0" cellpadding="0" cellspacing="0" width="500" style="padding: 20px 0" class="responsive-table" {
                            tbody {
                                tr {
                                    td align="left" class="padding-copy" style="padding: 0; padding-right: 15px" {
                                        img src="cid:logo" height="55px" width="55px";
                                    }
                                    td align="left" class="padding-copy" style="padding: 0; font-size: 25px; font-weight: normal; color: #ffffff; width: 100%" {
                                        "Bambushain"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn content(message: Markup) -> Markup {
    html! {
        tr {
            td {
                table width="100%" border="0" cellspacing="0" cellpadding="0" {
                    tbody {
                        tr {
                            td align="center" class="padding-copy" style="font-size: 25px; color: #333333; padding-top: 0px" {
                            "Passwort vergessen"
                                br;
                            }
                        }
                        tr {
                            td align="left" class="padding-copy textlightStyle" style="padding: 20px 0 0 0; font-size: 16px; line-height: 25px; color: #333333" {
                                (message)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn mail(
    title: impl Into<String> + Clone,
    message: Markup,
    action_label: Option<impl Into<String> + Clone>,
    action_link: Option<impl Into<String> + Clone>,
) -> String {
    html! {
       (DOCTYPE)
        html lang="de" style="font-family: system-ui,-apple-system,'Segoe UI','Roboto','Ubuntu','Cantarell','Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji';" {
            (head(title.into()))
            body style="margin: 0; padding: 0" bgcolor="#ffffff" align="center" {
                (mail_title())
                table border="0" cellpadding="0" cellspacing="0" width="100%" id="ko_onecolumnBlock_1" {
                    tbody {
                        tr class="row-a" {
                            td bgcolor="#e3ede9" align="center" class="section-padding" style="padding-top: 30px; padding-left: 15px; padding-bottom: 30px; padding-right: 15px" {
                                table border="0" cellpadding="0" cellspacing="0" width="500" class="responsive-table" {
                                    tbody {
                                        tr {
                                            td {
                                                table width="100%" border="0" cellspacing="0" cellpadding="0" {
                                                    tbody {
                                                        (content(message))
                                                        @if let (Some(action_link), Some(action_label)) = (action_link, action_label) {
                                                            (action(action_label, action_link))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }.into_string()
}
