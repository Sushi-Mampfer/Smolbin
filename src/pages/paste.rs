use chrono::{TimeDelta, Utc};
use leptos::{component, prelude::*, task::spawn_local, view, IntoView};
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::{api::get_paste, datatypes::PasteType};

#[component]
pub fn PastePage() -> impl IntoView {
    let content = signal("".to_string());
    let query = use_params_map();

    Effect::new(move |_| {
        let navigate = use_navigate();
        spawn_local(async move {
            let paste = match get_paste(query.get().get("id").unwrap()).await {
                Ok(p) => match p {
                    Some(p) => p,
                    None => {
                        navigate("/", Default::default());
                        return;
                    }
                },
                _ => {
                    navigate("/", Default::default());
                    return;
                }
            };
            match paste.paste_type {
                PasteType::Text => content.1.set(paste.content),
                PasteType::Url => navigate(&paste.content, Default::default()),
                _ => navigate("/", Default::default()),
            }
        })
    });

    view! {
        <textarea bind:value=content readonly></textarea>
    }
}
