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
    pub async fn get_by_id(id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Setlist, "SELECT * FROM setlists WHERE id = $1", id)
            .fetch_one(crate::database::get_db())
            .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get() -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Setlist, "SELECT * FROM setlists WHERE title = 'Oefenen'")
            .fetch_one(crate::database::get_db())
            .await
    }

    #[cfg(feature = "ssr")]
    pub async fn set_songs(songs: Vec<i32>) -> Result<(), sqlx::Error> {
        let existing_songs = Setlist::get().await?.songs;
        let new_songs = songs
            .iter()
            .filter(|s| !existing_songs.contains(s))
            .map(|s| s.to_owned())
            .collect::<Vec<i32>>();

        match Setlist::get().await {
            Ok(setlist) => {
                if setlist.is_locked {
                    return Err(sqlx::Error::RowNotFound);
                };
                sqlx::query!(
                    "UPDATE setlists SET songs = array_cat(songs, $1) WHERE id = 1",
                    &new_songs
                )
                .execute(crate::database::get_db())
                .await
                .map(|_| ())
            }
            Err(e) => return Err(e),
        }
    }

    #[cfg(feature = "ssr")]
    pub async fn clean() -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE setlists SET songs = '{}' WHERE id = 1")
            .execute(crate::database::get_db())
            .await
            .map(|_| ())
    }

    #[cfg(feature = "ssr")]
    pub async fn song_in_setlist(song_id: i32) -> Result<bool, sqlx::Error> {
        let setlist = Setlist::get().await?;
        Ok(setlist.songs.contains(&song_id))
    }
}
