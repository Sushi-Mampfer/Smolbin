pub mod api;
pub mod app;
pub mod datatypes;
pub mod pages;
pub mod components;
mod raw;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use crate::{app::*, datatypes::AppState};
    use axum::{Extension, Router, routing::get};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::{query, sqlite::SqliteConnectOptions, SqlitePool};
    use tokio::spawn;
    use std::{env, str::FromStr};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::from_str("sqlite://db.sqlite")
            .unwrap()
            .create_if_missing(true),
    )
    .await
    .unwrap();

    query(
        r#"
        CREATE TABLE IF NOT EXISTS pastes (
            id varchar(8) PRIMARY KEY,
            content TEXT NOT NULL,
            expiery INT NOT NULL,
            type INT NOT NULL
        )
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let pool_pass = pool.clone();

    spawn(async move {
        use std::time::Duration;
        use tokio::time::interval;

        let mut interval = interval(Duration::from_secs(1));
        loop {
            use chrono::Utc;

            interval.tick().await;
            match query(
                r#"
                DELETE FROM pastes
                WHERE expiery < ? AND expiery > 0
            "#).bind(Utc::now().timestamp()).execute(&pool_pass).await  {
                    Ok(r) => {
                        let deleted = r.rows_affected();
                        if deleted > 0 {
                            println!("Deleted {} expired pastes", deleted)
                        }
                    }
                    Err(e) => eprintln!("An unexpected error occured: {}", e.to_string()),
                }
        }
    });

    let state = AppState { pool };
    let state_pass = state.clone();

    let app = Router::new()
        .route("/raw/{id}", get(raw::raw))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(state_pass.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(Extension(state))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    ))
    .await
    .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
