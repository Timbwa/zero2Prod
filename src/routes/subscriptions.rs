use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    _form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    // We use get_ref to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
