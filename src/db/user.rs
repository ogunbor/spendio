pub async fn has_with_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    // Runtime query to check if a user with the given email exists
    sqlx::query("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(db)
        .await
        .ok()
        .is_some()
}
