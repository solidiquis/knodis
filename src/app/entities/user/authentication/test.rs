#[tokio::test]
async fn authenticate_via_pw() {
    use crate::box_fut;
    use crate::internal::db::pg::Pg;
    use super::authenticate_via_pw;
    use super::super::builder::{UserBuilder, error::UserBuilderError};
    use super::super::User;
    
    let pg = Pg::new().await;

    let _ = pg.with_conn(|conn| box_fut!({
        let username = "cthuwu";
        let password = "nyarlathotep123";
        let email = "cthulhu@ryleh.com";

        let new_user = UserBuilder::default()
            .email(&email).unwrap()
            .password(&password)
            .username(&username)
            .build(conn)
            .await?;

        let user = authenticate_via_pw(conn, username, password).await;

        if let Some(authenticated_user) = user {
            assert_eq!(new_user.id, authenticated_user.id);
        } else {
            println!("Failed to authenticate user.");
            assert!(false);
        }

        Err::<User, UserBuilderError>(UserBuilderError::Rollback)
    })).await;
}
