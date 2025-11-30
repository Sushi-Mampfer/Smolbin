use crate::pages::{HomePage, PastePage};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html class="w-full h-full" lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <Stylesheet id="leptos" href="/pkg/bin.css" />
                <MetaTags />
            </head>
            <body class="w-full h-full">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="SmolBin" />
        <Router>
            <main class="w-full h-full bg-gray-800 p-8">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("/paste/:id") view=PastePage />
                </Routes>
            </main>
        </Router>
    }
}
