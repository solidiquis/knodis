#[tokio::test]
async fn test_user_builder() {
    use crate::internal::db::pg::Pg;
    use sqlx::Connection;
    use super::{UserBuilder, UserBuilderError};
    use super::User;

    let pool = Pg::new().await;

    let mut conn = pool.acquire().await.unwrap();

    let _ = conn.transaction(|conn| Box::pin(async move {
        let user = UserBuilder::default()
            .email("cthulhu@ryleh.com").unwrap()
            .password("nyarlathotep123")
            .username("cthuwu")
            .build(conn)
            .await?;

        assert_eq!(user.email, Some("cthulhu@ryleh.com".to_owned()));
        assert_eq!(user.username, "cthuwu");
        assert_ne!(user.password, Some("nyarlathotep123".to_owned()));

        Err::<User, UserBuilderError>(UserBuilderError::Rollback)
    })).await;
}
