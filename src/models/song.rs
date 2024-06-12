use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::components::edit_song::EditSongData;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct Rehearsal {
    pub unselected_songs: Vec<Song>,
    pub selected_songs: Vec<Song>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub release_mid: Option<String>,
    pub artist_mid: Option<String>,
    pub last_played_at: Option<NaiveDate>,
    pub bpm: Option<i32>,
    pub gs_url: Option<String>,
    pub lyrics: String,
}

#[cfg(feature = "ssr")]
type Result<T> = std::result::Result<T, sqlx::Error>;

impl Song {
    #[cfg(feature = "ssr")]
    pub async fn get(song_id: i32) -> Result<Self> {
        sqlx::query!(
            "
        SELECT 
          * 
        FROM songs as s 
        WHERE s.id = $1",
            song_id
        )
        .map(|row| Song {
            id: row.id,
            artist: row.artist,
            title: row.title,
            last_played_at: row.last_played_at,
            release_mid: row.release_mid,
            artist_mid: row.artist_mid,
            gs_url: row.gs_url,
            bpm: row.bpm,
            lyrics: row.lyrics,
        })
        .fetch_one(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all_in_setlist(setlist_id: i32) -> Result<Vec<Song>> {
        use super::setlist::Setlist;
        let setlist_songs = Setlist::get_by_id(setlist_id).await?.songs;

        let all = Self::get_all().await?;
        Ok(all
            .into_iter()
            .filter(|song| setlist_songs.contains(&song.id))
            .collect())
    }

    #[cfg(feature = "ssr")]
    pub async fn get_rehearsal() -> Result<Rehearsal> {
        let selected = Self::get_all_in_setlist(1).await?;
        let unselected = Self::get_all()
            .await?
            .into_iter()
            .filter(|song| !selected.contains(song))
            .collect();
        Ok(Rehearsal {
            selected_songs: selected,
            unselected_songs: unselected,
        })
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all() -> Result<Vec<Self>> {
        sqlx::query!(
            "SELECT 
              * 
            FROM songs as s 
            ORDER BY 
              s.title ASC"
        )
        .map(|row| Song {
            id: row.id,
            artist: row.artist,
            title: row.title,
            gs_url: row.gs_url,
            last_played_at: row.last_played_at,
            release_mid: row.release_mid,
            artist_mid: row.artist_mid,
            bpm: row.bpm,
            lyrics: row.lyrics,
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn set_played(song_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE songs SET last_played_at = CURRENT_DATE WHERE id = $1",
            song_id
        )
        .execute(crate::database::get_db())
        .await
        .map(|_| ())
    }

    #[cfg(feature = "ssr")]
    pub async fn update(data: EditSongData) -> Result<()> {
        sqlx::query!(
            "UPDATE songs SET artist=$2, title=$3, bpm=$4, lyrics = $5 WHERE id = $1",
            data.id,
            data.artist,
            data.title,
            data.bpm,
            data.lyrics
        )
        .execute(crate::database::get_db())
        .await
        .map(|_| ())
    }
}
