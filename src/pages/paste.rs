use leptos::{IntoView, component, prelude::*, task::spawn_local, view};
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::{api::get_paste, datatypes::PasteType};

#[component]
pub fn PastePage() -> impl IntoView {
    let content = signal("".to_string());

    let params = use_params_map();

    Effect::new(move |_| {
        let navigate = use_navigate();
        let id = params.get().get("id").unwrap();
        spawn_local(async move {
            let paste = match get_paste(id).await {
                Ok(p) => match p {
                    Some(p) => p,
                    None => {
                        navigate("/?e=Paste%20not%20found", Default::default());
                        return;
                    }
                },
                Err(e) => {
                    navigate(&format!("/?e={}", e.to_string()), Default::default());
                    return;
                }
            };
            match paste.paste_type {
                PasteType::Text => content.1.set(paste.content),
                PasteType::Url => navigate(&paste.content, Default::default()),
                _ => navigate("/?e=Paste%20type%20not%20supported", Default::default()),
            }
        })
    });

    view! {
        <div class="grid grid-cols-[25%_50%_25%] grid-rows-[3rem_1fr] underline">
            <a class="text-gray-300 col-span-1" href="/">
                Smolbin
            </a>

            <a
                class="text-gray-300 col-span-1 col-start-3 justify-self-end underline"
                href=move || { format!("/raw/{}", params.get().get("id").unwrap()) }
                target="_self"
            >
                Raw
            </a>
            <textarea
                class="text-gray-300 resize-none col-span-1 col-start-2 bg-gray-600 rounded-xl p-4 focus:outline-none"
                bind:value=content
                readonly
            ></textarea>
        </div>
    }
}
