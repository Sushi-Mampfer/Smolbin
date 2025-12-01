use axum::{Extension, extract::Path, response::{IntoResponse, Redirect, Response}};
use sqlx::Row;

use crate::{datatypes::{Paste, PasteType, AppState}};


pub async fn raw(Extension(state): Extension<AppState>, Path(id): Path<String>) -> Response {
    let Some(paste) = (match sqlx::query(
        r#"
            SELECT * FROM pastes
            WHERE id = ?
    "#,
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await {
        Ok(r) => r,
        Err(e) => return format!("Databese error: {}", e).into_response(),
    }) else {
        return "404, not found".into_response();
    };
    if paste.get::<i32, &str>("expiry") == -1 {
        let _ = sqlx::query(
            r#"
            DELETE FROM pastes
            WHERE id = ?
        "#,
        )
        .bind(id)
        .execute(&state.pool)
        .await;
    }

    let paste = Paste {
        id: paste.get("id"),
        content: paste.get("content"),
        paste_type: PasteType::from(paste.get::<u8, &str>("type")),
    };
    match paste.paste_type {
        crate::datatypes::PasteType::Text => paste.content.into_response(),
        crate::datatypes::PasteType::Url => Redirect::to(&paste.content).into_response(),
    }
}