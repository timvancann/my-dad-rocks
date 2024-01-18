use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Setlist {
    pub id: i32,
    pub title: String,
    pub is_locked: bool,
    pub songs: Vec<i32>,
}

impl Setlist {
    #[cfg(feature = "ssr")]
    pub async fn update_lock(locked: bool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE setlists SET is_locked = $1 WHERE title = 'Oefenen'",
            locked
        )
        .execute(crate::database::get_db())
        .await
        .map(|_| ())
    }

    #[cfg(feature = "ssr")]
    pub async fn get() -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Setlist, "SELECT * FROM setlists WHERE title = 'Oefenen'")
            .fetch_one(crate::database::get_db())
            .await
    }

    #[cfg(feature = "ssr")]
    pub async fn set_songs(songs: Vec<i32>) -> Result<(), sqlx::Error> {
        match Setlist::get().await {
            Ok(setlist) => {
                if setlist.is_locked {
                    return Err(sqlx::Error::RowNotFound);
                };
                sqlx::query!("UPDATE setlists SET songs = $1 WHERE id = 1", &songs)
                    .execute(crate::database::get_db())
                    .await
                    .map(|_| ())
            }
            Err(e) => return Err(e),
        }
    }
}
