use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct Rehearsal {
    pub unselected_songs: Vec<Song>,
    pub selected_songs: Vec<Song>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub sanitized_title: String,
    pub last_played_at: Option<NaiveDate>,
    pub gs_url: Option<String>,
    pub should_play: bool,
    pub lyrics: String,
}

impl Default for Song {
    fn default() -> Self {
        Song {
            id: 0,
            artist: "".to_string(),
            title: "".to_string(),
            last_played_at: None,
            gs_url: None,
            sanitized_title: "".to_string(),
            should_play: false,
            lyrics: "".to_string(),
        }
    }
}

#[cfg(feature = "ssr")]
type Result<T> = std::result::Result<T, sqlx::Error>;

impl Song {
    #[cfg(feature = "ssr")]
    pub async fn get(song_id: i32) -> Result<Self> {
        use super::setlist::Setlist;

        let song_in_setlist = Setlist::song_in_setlist(song_id).await?;

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
            sanitized_title: row.sanitized_title.clone(),
            gs_url: row.gs_url,
            should_play: song_in_setlist,
            lyrics: row.lyrics,
        })
        .fetch_one(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all_in_setlist(setlist_id: i32) -> Result<Vec<Song>> {
        use super::setlist::Setlist;
        let setlist_songs = Setlist::get_by_id(setlist_id).await?.songs;
        sqlx::query!(
            "SELECT 
              * 
            FROM songs"
        )
        .map(|row| Song {
            id: row.id,
            artist: row.artist,
            title: row.title,
            last_played_at: row.last_played_at,
            gs_url: row.gs_url,
            sanitized_title: row.sanitized_title.clone(),
            should_play: setlist_songs.contains(&row.id),
            lyrics: row.lyrics,
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_rehearsal() -> Result<Rehearsal> {
        Self::get_all().await.map(|songs| {
            let mut selected = Vec::default();
            let mut unselected = Vec::default();
            for song in songs {
                if song.should_play {
                    selected.push(song);
                } else {
                    unselected.push(song);
                }
            }
            Rehearsal {
                selected_songs: selected,
                unselected_songs: unselected,
            }
        })
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all() -> Result<Vec<Self>> {
        use super::setlist::Setlist;

        let setlist_songs = Setlist::get().await?.songs;
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
            sanitized_title: row.sanitized_title.clone(),
            should_play: setlist_songs.contains(&row.id),
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
    pub async fn update_lyrics(id: i32, lyrics: String) -> Result<()> {
        sqlx::query!("UPDATE songs SET lyrics = $1 WHERE id = $2", lyrics, id)
            .execute(crate::database::get_db())
            .await
            .map(|_| ())
    }
}

pub enum ThumbnailType {
    Thumbnail,
    Player,
}
