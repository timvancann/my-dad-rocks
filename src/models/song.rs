use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub last_played_at: Option<NaiveDate>,
    pub audio_file_path: String,
    pub created_at: NaiveDate,
}

impl Song {
    #[cfg(feature = "ssr")]
    pub async fn get(song_id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Song,
            "
        SELECT 
          * 
        FROM songs as s 
        WHERE s.id = $1",
            song_id
        )
        .fetch_one(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all() -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Song,
            "SELECT 
              * 
            FROM songs as s 
            ORDER BY 
              s.last_played_at ASC NULLS FIRST, s.title ASC"
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
}
