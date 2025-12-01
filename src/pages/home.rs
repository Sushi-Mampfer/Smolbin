use std::collections::BTreeMap;

use chrono::{TimeDelta, Utc};
use leptos::{IntoView, component, prelude::*, task::spawn_local, view};
use leptos_router::hooks::{use_navigate, use_query_map};

use crate::{api::create_paste, components::ErrorPopup, datatypes::PasteType};

#[component]
pub fn HomePage() -> impl IntoView {
    let error = signal("".to_string());
    let content = signal("".to_string());
    let paste_type = signal("0".to_string());
    let expiry = signal("0".to_string());
    let (type_select, set_type_select) = signal(false);
    let (show_info, set_show_info) = signal(false);

    let mut paste_types = BTreeMap::new();
    paste_types.insert(0, "Text".to_string());
    paste_types.insert(1, "Url".to_string());

    let query = use_query_map();

    Effect::new(move || {
        if let Some(e) = query.get().get("e") {
            error.1.set(e);
        }
    });

    let create = move |_| {
        let content = content.0.get();
        if content.is_empty() {
            error.1.set("Don't waste my storage".to_string());
            return;
        }
        let paste_type = match paste_type.0.get_untracked().as_str() {
            "0" => PasteType::Text,
            "1" => {
                if !content.starts_with("http://") && !content.starts_with("https://") {
                    error.1.set("Invalid url".to_string());
                    return;
                }
                PasteType::Url
            },
            _ => return,
        };
        let expiry = match expiry.0.get_untracked().as_str() {
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
                            error.1.set("Invalid expiry".to_string());
                            return;
                        },
                    };
                    delay += mult
                        * match i[..i.len() - 1].parse::<i64>() {
                            Ok(t) => t,
                            _ => {
                                error.1.set("Invalid expiry".to_string());
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
            match create_paste(content, paste_type, expiry)
                .await {
                    Ok(id) => use_navigate()(&format!("/paste/{}", id), Default::default()),
                    Err(e) => error.1.set(e.to_string()),
                }
            
        });
    };

    view! {
        <ErrorPopup error=error />
        <div class="grid grid-cols-[1fr_auto_1fr] grid-rows-[auto_auto_1fr] w-full h-full">
            <p class="text-gray-300 col-start-2 text-center h-20 leading-20 text-4xl">Smolbin</p>
            <div class="grid grid-cols-1 grid-rows-[auto_auto_auto_auto] row-start-2 col-start-2 gap-2">
                <textarea
                    placeholder="Content or Url"
                    class="text-gray-300 focus:outline-none bg-gray-600 p-4 rounded-4xl resize-none w-150 h-75"
                    bind:value=content
                />
                <p
                    class="text-gray-300 focus:outline-none w-[25%] m-auto p-4 flex items-center justify-center"
                    on:click=move |_| set_type_select.update(|t| *t = !*t)
                >
                    <span>
                        {{
                            let value = paste_types.clone();
                            move || {
                                value.get(&paste_type.0.get().parse().unwrap()).unwrap().to_owned()
                            }
                        }}
                    </span>
                    <Show
                        when=move || type_select.get()
                        fallback=move || {
                            view! {
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="#d1d5dc"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="lucide lucide-chevron-down-icon lucide-chevron-down"
                                >
                                    <path d="m6 9 6 6 6-6" />
                                </svg>
                            }
                        }
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="#d1d5dc"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-chevron-up-icon lucide-chevron-up"
                        >
                            <path d="m18 15-6-6-6 6" />
                        </svg>
                    </Show>
                </p>
                <Show when=move || type_select.get()>
                    <div class="text-gray-300 absolute top-115 text-center w-38 p-4 bg-gray-700 left-[calc(50%-76px)] rounded-2xl hover:cursor-pointer z-100">
                        {{
                            let value = paste_types.clone();
                            move || {
                                value
                                    .iter()
                                    .map(|t| {
                                        let t0 = t.0.to_string();
                                        let t1 = t.1.clone();
                                        view! {
                                            <p
                                                class="text-center w-full hover:cursor-pointer"
                                                on:click=move |_| {
                                                    paste_type.1.set(t0.to_string());
                                                    set_type_select.set(false)
                                                }
                                            >
                                                {t1}
                                            </p>
                                        }
                                    })
                                    .collect_view()
                            }
                        }}
                    </div>
                </Show>
                <div class="relative w-[25%] m-auto">
                    <input
                        placeholder="expiry"
                        class="text-gray-300 focus:outline-none text-center w-full m-auto bg-gray-600 p-4 rounded-4xl"
                        type="text"
                        bind:value=expiry
                    />
                    <svg
                        on:mouseover=move |_| set_show_info.set(true)
                        on:mouseout=move |_| set_show_info.set(false)
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="#d1d5dc"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="lucide lucide-info-icon lucide-info absolute right-4 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400 hover:cursor-pointer"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <path d="M12 16v-4" />
                        <path d="M12 8h.01" />
                    </svg>
                    <Show when=move || show_info.get()>
                        <div class="text-gray-300 absolute -right-42 top-1/2 -translate-y-1/2 text-center w-38 p-4 bg-gray-600 rounded-2xl hover:cursor-pointer z-100 pointer-events-none">
                            <p>
                                Use -1 (never), 0 (read once), or durations like "2h", "7d", "1w 3d"
                                Units: s, m, h, d, w, M, y
                            </p>

                        </div>
                    </Show>
                </div>
                <button
                    class="text-gray-300 focus:outline-none hover:cursor-pointer bg-gray-600 p-4 rounded-4xl w-[25%] m-auto"
                    on:click=create
                >
                    Create
                </button>
            </div>
        </div>
    }
}
