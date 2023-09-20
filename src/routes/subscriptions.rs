use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let uuid = Uuid::new_v4();
    log::info!(
        "request id {uuid} - Adding {} {} as a new subscriber",
        form.email,
        form.name
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(db_pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!("request id {uuid} - New subscriber details have been saved",);
            return HttpResponse::Ok().finish();
        }
        Err(e) => {
            log::error!("request id {uuid} - Failed to execute query: {:#?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
