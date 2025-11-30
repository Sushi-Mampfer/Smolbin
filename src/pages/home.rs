use chrono::{TimeDelta, Utc};
use leptos::{IntoView, component, prelude::*, task::spawn_local, view};
use leptos_router::hooks::{use_navigate, use_query_map};

use crate::{api::create_paste, components::ErrorPopup, datatypes::PasteType};

#[component]
pub fn HomePage() -> impl IntoView {
    let error = signal("".to_string());
    let content = signal("".to_string());
    let paste_type = signal("".to_string());
    let expiery = signal("0".to_string());

    let query = use_query_map();

    Effect::new(move || {
        if let Some(e) = query.get().get("e") {
            error.1.set(e);
        }
    });

    let create = move |_| {
        let paste_type = match paste_type.0.get_untracked().as_str() {
            "0" => PasteType::Text,
            "1" => PasteType::Url,
            _ => return,
        };
        let expiery = match expiery.0.get_untracked().as_str() {
            "-1" => -1,
            "0" => 0,
            e => {
                let parts = e.split(" ");
                let mut delay = 0;
                for i in parts {
                    let mult = match i.chars().last().unwrap() {
                        's' => 1,
                        'm' => 60,
                        'h' => 60 * 60,
                        'd' => 24 * 60 * 60,
                        'w' => 7 * 24 * 60 * 60,
                        'M' => 30 * 24 * 60 * 60,
                        'y' => 365 * 24 * 60 * 60,
                        _ => {
                            error.1.set("Invalid expiery".to_string());
                            return;
                        },
                    };
                    delay += mult
                        * match i[..i.len() - 1].parse::<i64>() {
                            Ok(t) => t,
                            _ => {
                                error.1.set("Invalid expiery".to_string());
                                return;
                            },
                        }
                }
                
                Utc::now()
                    .checked_add_signed(TimeDelta::seconds(delay))
                    .unwrap()
                    .timestamp() as i32
            }
        };

        spawn_local(async move {
            match create_paste(content.0.get_untracked(), paste_type, expiery)
                .await {
                    Ok(id) => use_navigate()(&format!("/paste/{}", id), Default::default()),
                    Err(e) => error.1.set(e.to_string()),
                }
            
        });
    };

    view! {
        <ErrorPopup error=error />

        <input type="text" bind:value=content />
        <select bind:value=paste_type>
            <option value="0">Text</option>
            <option value="1">Url</option>
        </select>
        <input type="text" bind:value=expiery />
        <button on:click=create>Create</button>
    }
}
