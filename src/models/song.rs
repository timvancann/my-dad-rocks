use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub last_played_at: Option<NaiveDate>,
    pub audio_file_path: String,
    pub practice_next: bool,
}

impl Song {
    #[cfg(feature = "ssr")]
    pub async fn get(song_id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Song, "SELECT id, artist, title, last_played_at, audio_file_path, practice_next FROM songs as s WHERE s.id = $1", song_id)
            .fetch_one(crate::database::get_db())
            .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all() -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Song,
            "SELECT id, artist, title, last_played_at, audio_file_path, practice_next FROM songs as
         s ORDER BY s.last_played_at ASC NULLS FIRST, s.title ASC"
        )
        .fetch_all(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn set_played(song_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE songs SET last_played_at = CURRENT_DATE WHERE id = $1",
            song_id
        )
        .execute(crate::database::get_db())
        .await
        .map(|_| ())
    }

    #[cfg(feature = "ssr")]
    pub async fn set_to_practice_next(&self, to_practice: bool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE songs SET practice_next = $1 WHERE id = $2",
            to_practice,
            self.id,
        )
        .execute(crate::database::get_db())
        .await
        .map(|_| ())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>,
}
