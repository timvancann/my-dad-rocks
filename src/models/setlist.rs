use crate::models::song::Song;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Setlist {
    pub id: i32,
    pub title: String,
    pub is_locked: bool,
    pub songs: Vec<Song>,
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
        let songs = sqlx::query_as!(
            Song,
            "SELECT 
              id, 
              title, 
              artist, 
              last_played_at, 
              audio_file_path,
              created_at
            FROM (SELECT unnest(songs) song_id FROM setlists WHERE title = 'Oefenen') 
            as a LEFT JOIN songs b on b.id=a.song_id
            ORDER BY b.title ASC
            "
        )
        .fetch_all(crate::database::get_db())
        .await?;

        sqlx::query!("SELECT * FROM setlists WHERE title = 'Oefenen'")
            .map(|row| Setlist {
                id: row.id,
                title: row.title,
                is_locked: row.is_locked,
                songs: songs.clone(),
            })
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
