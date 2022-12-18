#[tokio::test]
async fn test_user_builder() {
    use crate::box_fut;
    use crate::internal::db::pg::Pg;
    use super::{UserBuilder, UserBuilderError};
    use super::User;

    let pg = Pg::new().await;

    let _ = pg.transaction(|conn| box_fut!({
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
