use leptos::{component, prelude::*, view, IntoView};

#[component]
pub fn ErrorPopup(error: (ReadSignal<String>, WriteSignal<String>)) -> impl IntoView {
    let (show, set_show) = signal(false);
    Effect::new(move || {
        set_show.set(!error.0.get().is_empty());
    });
    view! {
        <div
            class="absolute right-0 bg-purple-700 hover:cursor-pointer h-30 w-60 duration-400 m-4 p-4 rounded-2xl -top-28"
            style:top=move || if show.get() { "0" } else { "-9.5rem" }
            on:click=move |_| set_show.set(false)
        >
            <h1 class="text-center overflow-y-scroll text-gray-300 font-semibold">{error.0}</h1>
        </div>
    }
}
