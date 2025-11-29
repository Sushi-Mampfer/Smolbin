use chrono::{TimeDelta, Utc};
use leptos::{component, prelude::*, task::spawn_local, view, IntoView};

use crate::{api::create_paste, datatypes::PasteType};

#[component]
pub fn HomePage() -> impl IntoView {
    let content = signal("".to_string());
    let paste_type = signal("".to_string());
    let expiery = signal("0".to_string());

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
                let mut delay: i32 = 0;
                for i in parts {
                    let mult = match i.chars().last().unwrap() {
                        's' => 1,
                        'm' => 60,
                        'h' => 60 * 60,
                        'd' => 24 * 60 * 60,
                        'w' => 7 * 24 * 60 * 60,
                        'M' => 30 * 24 * 60 * 60,
                        'y' => 365 * 24 * 60 * 60,
                        _ => return,
                    };
                    delay += mult
                        * match i[..1 - i.len() - 1].parse::<i32>() {
                            Ok(t) => t,
                            _ => return,
                        }
                }
                Utc::now()
                    .checked_sub_signed(TimeDelta::seconds(delay as i64))
                    .unwrap()
                    .timestamp() as i32
            }
        };

        spawn_local(async move {
            create_paste(content.0.get_untracked(), paste_type, expiery)
                .await
                .unwrap();
        });
    };

    view! {
        <input type="text" bind:value=content />
        <select bind:value=paste_type>
            <option value="0">Text</option>
            <option value="1">Url</option>
        </select>
        <input type="number" bind:value=expiery/>
        <button on:click=create>Create</button>
    }
}
