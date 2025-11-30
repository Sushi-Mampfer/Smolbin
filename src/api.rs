use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

use crate::datatypes::{Paste, PasteType};

#[server]
pub async fn get_paste(id: String) -> Result<Option<Paste>, ServerFnError> {
    use sqlx::Row;

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
    if paste.get::<i32, &str>("expiery") == -1 {
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
    expiery: i32,
) -> Result<String, ServerFnError> {
    use rand::distr::SampleString;

    let state = expect_context::<crate::datatypes::AppState>();
    for _ in 0..100 {
        let id = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 8);
        match sqlx::query(
            r#"
            INSERT INTO pastes (id, content, expiery, type)
            VALUES (?, ?, ?, ?)
        "#,
        )
        .bind(&id)
        .bind(&content)
        .bind(expiery)
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
