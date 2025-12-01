use leptos::{
    prelude::ServerFnError,
    server,
};

use crate::datatypes::{Paste, PasteType};

#[server]
pub async fn get_paste(id: String) -> Result<Option<Paste>, ServerFnError> {
    use sqlx::Row;
    use leptos::prelude::expect_context;

    let state = expect_context::<crate::datatypes::AppState>();
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
        Err(e) => return Err(ServerFnError::ServerError(format!("Databese error: {}", e))),
    }) else {
        return Ok(None);
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

    return Ok(Some(paste));
}

#[server]
pub async fn create_paste(
    content: String,
    paste_type: PasteType,
    expiry: i32,
) -> Result<String, ServerFnError> {
    if paste_type == PasteType::Url {
        if !content.starts_with("http://") && !content.starts_with("https://") {
            return Err(ServerFnError::ServerError("Invalid url".to_string()));
        }
    }
    use rand::distr::SampleString;
    use leptos::prelude::expect_context;

    let state = expect_context::<crate::datatypes::AppState>();
    for _ in 0..100 {
        let id = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 8);
        match sqlx::query(
            r#"
            INSERT INTO pastes (id, content, expiry, type)
            VALUES (?, ?, ?, ?)
        "#,
        )
        .bind(&id)
        .bind(&content)
        .bind(expiry)
        .bind(paste_type.clone() as u8)
        .execute(&state.pool)
        .await
        {
            Ok(_) => return Ok(id),
            Err(_) => continue,
        }
    }

    return Err(ServerFnError::ServerError(
        "No space left on device".to_string(),
    ));
}
