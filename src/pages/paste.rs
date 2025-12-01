use leptos::{IntoView, component, prelude::*, task::spawn_local, view};
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::{api::get_paste, datatypes::PasteType};

#[component]
pub fn PastePage() -> impl IntoView {
    let content = signal("".to_string());
    let (link, set_link) = signal(false);

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
                PasteType::Text => {
                    set_link.set(false);
                    content.1.set(paste.content)
                },
                PasteType::Url => {
                    set_link.set(true);
                    content.1.set(paste.content)
                },
            }
        })
    });

    view! {
        <div class="grid grid-cols-[25%_50%_25%] grid-rows-[3rem_1fr] h-full">
            <a class="text-gray-300 col-span-1 text-3xl text-center underline w-full" href="/">
                Smolbin
            </a>

            <a
                class="text-gray-300 col-span-1 col-start-3 justify-self-end text-3xl text-center underline w-full"
                href=move || { format!("/raw/{}", params.get().get("id").unwrap()) }
                target="_self"
            >
                {move || { if link.get() { "Direkt Link" } else { "Raw" } }}
            </a>
            <textarea
                class="text-gray-300 resize-none col-span-1 col-start-2 bg-gray-600 rounded-xl p-4 focus:outline-none overflow-y-scroll hover:cursor-pointer"
                bind:value=content
                on:click=move |_| {
                    if link.get() {
                        use_navigate()(&content.0.get(), Default::default())
                    }
                }
                readonly
            ></textarea>
        </div>
    }
}
