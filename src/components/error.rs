use leptos::{component, prelude::*, view, IntoView};

#[component]
pub fn ErrorPopup(error: (ReadSignal<String>, WriteSignal<String>)) -> impl IntoView {
    view! {
        <Show when=move || { !error.0.get().is_empty() }>
            <div on:click=move |_| error.1.set("".to_string())>
                <h1>{error.0}</h1>
            </div>
        </Show>
    }
}
